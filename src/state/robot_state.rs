#[derive(Clone)]
pub struct RobotState {
    pub x: f64,
    pub y: f64,
    pub velocity: f64,
    pub heading: f64,
}

impl RobotState {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            velocity: 0.4,
            heading: 0.0,
        }
    }
}