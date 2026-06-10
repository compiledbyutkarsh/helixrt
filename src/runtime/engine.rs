use tokio::sync::{
    broadcast,
    mpsc,
};
use tokio::time::{sleep, Duration};

use crate::autonomy::navigator::WaypointNavigator;
use crate::bus::command_bus::{
    CommandBus,
    RuntimeCommand,
};
use crate::fusion::localization::LocalizationEngine;
use crate::logging::runtime_logger::RuntimeLogger;
use crate::navigation::pathfinding::NavigationGrid;
use crate::network::telemetry::TelemetryServer;
use crate::persistence::map_store::MapStore;
use crate::robot::controller::RobotController;
use crate::sensors::lidar::LidarScanner;
use crate::simulation::world::SimulationWorld;
use crate::tasks::scheduler::RuntimeScheduler;
use crate::ws::server::WebSocketServer;

pub struct RuntimeEngine {
    telemetry: TelemetryServer,
    robots: Vec<RobotController>,
    world: SimulationWorld,
    lidar: LidarScanner,
    navigation: NavigationGrid,
    navigator: WaypointNavigator,
    tick_rate: u64,
}

impl RuntimeEngine {
    pub fn new() -> Self {
        let (
            telemetry_channel,
            _,
        ) = broadcast::channel::<String>(256);

        let websocket_channel =
            telemetry_channel.clone();

        tokio::spawn(async move {
            WebSocketServer::start(
                websocket_channel
            )
            .await;
        });

        Self {
            telemetry: TelemetryServer::new(
                telemetry_channel
            ),

            robots: vec![
                RobotController::new(),
                RobotController::new(),
            ],

            world: SimulationWorld::new(),

            lidar: LidarScanner::new(),

            navigation:
                NavigationGrid::new(
                    128,
                    128,
                ),

            navigator:
                WaypointNavigator::new(),

            tick_rate: 20,
        }
    }

    pub async fn boot(&mut self) {
        RuntimeLogger::info(
            "booting helix runtime"
        );

        self.world.initialize().await;

        self.telemetry.initialize().await;

        for robot in &self.robots {
            robot.initialize().await;
        }

        let (tx, mut rx) =
            mpsc::channel::<RuntimeCommand>(32);

        let bus = CommandBus::new(tx);

        tokio::spawn(async move {
            RuntimeScheduler::heartbeat().await;
        });

        tokio::spawn(async move {
            sleep(Duration::from_secs(90)).await;

            bus.dispatch(
                RuntimeCommand::Shutdown,
            )
            .await;
        });

        RuntimeLogger::info(
            "runtime online"
        );

        self.run_loop(&mut rx).await;
    }

    async fn run_loop(
        &mut self,
        receiver: &mut mpsc::Receiver<
            RuntimeCommand,
        >,
    ) {
        let frame_time =
            Duration::from_millis(
                1000 / self.tick_rate
            );

        loop {
            while let Ok(command) =
                receiver.try_recv()
            {
                match command {
                    RuntimeCommand::Shutdown => {
                        RuntimeLogger::warn(
                            "runtime shutdown received"
                        );

                        return;
                    }

                    RuntimeCommand::Pause => {}

                    RuntimeCommand::Resume => {}
                }
            }

            self.world.update().await;

            for (robot_id, robot)
                in self
                    .robots
                    .iter_mut()
                    .enumerate()
            {
                robot.update().await;

                let robot_state =
                    robot.snapshot();

                let scan =
                    self.lidar.scan(
                        &robot_state
                    );

                for point in scan {
                    let x =
                        point.x.abs()
                            as usize
                            % 128;

                    let y =
                        point.y.abs()
                            as usize
                            % 128;

                    self.navigation
                        .mark_obstacle(
                            x,
                            y,
                        );
                }

                let occupancy =
                    self.navigation
                        .occupancy_ratio();

                let path =
                    self.navigation
                        .find_path(
                            (0, 0),
                            (64, 64),
                        );

                let localized =
                    LocalizationEngine::estimate(
                        &robot_state
                    );

                let distance =
                    self.navigator
                        .distance_to_target(
                            &robot_state
                        );

                println!(
                    "[navigation] robot={} distance={:.2} localized=({:.2}, {:.2}) path_nodes={}",
                    robot_id,
                    distance,
                    localized.0,
                    localized.1,
                    path.len()
                );

                self.telemetry
                    .broadcast(
                        robot_id,
                        &robot_state,
                        occupancy,
                    )
                    .await;

                MapStore::persist(
                    occupancy
                );
            }

            sleep(frame_time).await;
        }
    }
}