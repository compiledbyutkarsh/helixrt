use crate::state::robot_state::RobotState;

pub struct WaypointNavigator {
    target_x: f64,
    target_y: f64,
}

impl WaypointNavigator {
    pub fn new() -> Self {
        Self {
            target_x: 40.0,
            target_y: 40.0,
        }
    }

    pub fn distance_to_target(
        &self,
        state: &RobotState,
    ) -> f64 {
        let dx = self.target_x - state.x;

        let dy = self.target_y - state.y;

        (dx * dx + dy * dy).sqrt()
    }
}