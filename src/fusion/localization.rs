use crate::state::robot_state::RobotState;

pub struct LocalizationEngine;

impl LocalizationEngine {
    pub fn estimate(
        state: &RobotState,
    ) -> (f64, f64) {
        let filtered_x =
            (state.x * 0.92) + 0.08;

        let filtered_y =
            (state.y * 0.92) + 0.08;

        (filtered_x, filtered_y)
    }
}