use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Telemetry {
    pub vehicle_id: Uuid,

    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub heading: f64,

    pub fuel_level: f64,
    pub engine_temperature: f64,
    pub battery_voltage: f64,

    pub tire_pressure: TirePressure,

    pub last_updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TirePressure {
    pub front_left: f64,
    pub front_right: f64,
    pub rear_left: f64,
    pub rear_right: f64,
}
