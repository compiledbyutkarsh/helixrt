use serde::Serialize;

#[derive(Serialize)]
pub struct TelemetryPacket {
    pub x: f64,
    pub y: f64,
    pub heading: f64,
    pub occupancy: f64,
    pub packets_sent: u64,
}