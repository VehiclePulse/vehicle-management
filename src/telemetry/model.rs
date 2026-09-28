use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Telemetry {
    #[serde(rename = "_id")]
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub vehicle_id: Uuid,

    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub heading: f64,

    pub fuel_level: f64,
    pub engine_temperature: f64,
    pub battery_level: f64,

    pub tire_pressure: TirePressure,

    pub last_updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct TirePressure {
    #[validate(range(min = 0.0, max = 100.0, message = "pressure value is not valid for tire"))]
    pub front_left: f64,

    #[validate(range(min = 0.0, max = 100.0, message = "pressure value is not valid for tire"))]
    pub front_right: f64,

    #[validate(range(min = 0.0, max = 100.0, message = "pressure value is not valid for tire"))]
    pub rear_left: f64,

    #[validate(range(min = 0.0, max = 100.0, message = "pressure value is not valid for tire"))]
    pub rear_right: f64,
}


#[derive(Deserialize, Clone, Validate)]
pub struct UpdateTelemetryRequest {
    #[validate(range(min = -90.0, max = 90.0, message = "latitude value is not valid"))]
    pub latitude: f64,

    #[validate(range(min = -180.0, max = 180.0, message = "longitude value is not valid"))]
    pub longitude: f64,

    #[validate(range(min = -20.0, max = 300.0, message = "speed value is not valid"))]
    pub speed: f64,

    #[validate(range(min = 0.0, max = 360.0, message = "heading value is not valid"))]
    pub heading: f64,

    #[validate(range(min = 0.0, max = 100.0, message = "fuel level value is not valid"))]
    pub fuel_level: f64,

    #[validate(range(min = -50.0, max = 200.0, message = "temperature value is not valid"))]
    pub engine_temperature: f64,

    #[validate(range(min = 0.0, max = 360.0, message = "battery level value is not valid"))]
    pub battery_level: f64,

    pub tire_pressure: TirePressure,
}