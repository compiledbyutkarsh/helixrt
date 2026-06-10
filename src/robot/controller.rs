use rand::Rng;

use crate::state::robot_state::RobotState;

pub struct RobotController {
    state: RobotState,
}

impl RobotController {
    pub fn new() -> Self {
        Self {
            state: RobotState::new(),
        }
    }

    pub async fn initialize(&self) {
        println!("robot controller initialized");
    }

    pub async fn update(&mut self) {
        let mut rng = rand::thread_rng();

        self.state.heading += rng.gen_range(-0.15..0.15);

        self.state.x +=
            self.state.velocity * self.state.heading.cos();

        self.state.y +=
            self.state.velocity * self.state.heading.sin();
    }

    pub fn snapshot(&self) -> RobotState {
        self.state.clone()
    }
}