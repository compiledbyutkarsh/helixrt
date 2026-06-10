use crate::state::robot_state::RobotState;
use crate::telemetry::packet::TelemetryPacket;
use crate::ws::server::TelemetryChannel;

pub struct TelemetryServer {
    packets_sent: u64,
    channel: TelemetryChannel,
}

impl TelemetryServer {
    pub fn new(
        channel: TelemetryChannel,
    ) -> Self {
        Self {
            packets_sent: 0,
            channel,
        }
    }

    pub async fn initialize(&self) {
        println!(
            "telemetry server initialized"
        );
    }

    pub async fn broadcast(
        &mut self,
        robot_id: usize,
        state: &RobotState,
        occupancy: f64,
    ) {
        self.packets_sent += 1;

        let packet = TelemetryPacket {
            x: state.x,
            y: state.y,
            heading: state.heading,
            occupancy,
            packets_sent: self.packets_sent,
        };

        let payload =
            serde_json::to_string(&packet)
                .unwrap();

        let _ =
            self.channel.send(
                payload.clone()
            );

        println!(
            "[robot:{}] {}",
            robot_id,
            payload
        );
    }
}