use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vehicle {
    #[serde(rename = "_id")]
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub id: Uuid,
    pub registration_number: String,
    pub make: String,
    pub model: String,
    pub year: u16,
    pub simulation: VehicleSimulation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VehicleSimulation {
    pub enabled: bool,
    pub initial_latitude: f64,
    pub initial_longitude: f64,
    pub initial_heading: f64,
}
