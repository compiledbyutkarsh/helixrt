use rand::Rng;

use crate::state::robot_state::RobotState;

#[derive(Clone)]
pub struct LidarPoint {
    pub x: f64,
    pub y: f64,
}

pub struct LidarScanner {
    range: f64,
    resolution: usize,
}

impl LidarScanner {
    pub fn new() -> Self {
        Self {
            range: 30.0,
            resolution: 96,
        }
    }

    pub fn scan(
        &self,
        robot_state: &RobotState,
    ) -> Vec<LidarPoint> {
        let mut rng = rand::thread_rng();

        let mut points = Vec::new();

        for i in 0..self.resolution {
            let angle =
                (i as f64 / self.resolution as f64)
                    * std::f64::consts::TAU;

            let distance =
                rng.gen_range(2.0..self.range);

            let x =
                robot_state.x + angle.cos() * distance;

            let y =
                robot_state.y + angle.sin() * distance;

            points.push(LidarPoint { x, y });
        }

        points
    }
}