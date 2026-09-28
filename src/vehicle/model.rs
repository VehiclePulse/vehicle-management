use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVehicleRequest {

    #[validate(length(min = 1, max = 14, message = "registration number doesn't satisfy defined length"))]
    pub registration_number: String,

    #[validate(length(min = 1, max = 20, message = "make doesn't satisfy defined length"))]
    pub make: String,

    #[validate(length(min = 1, max = 20, message = "model doesn't satisfy defined length"))]
    pub model: String,

    #[validate(range(min = 1990, max = 2100, message = "year doesn't satisfy defined range"))]
    pub year: u16,

    pub simulation: VehicleSimulation,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVehicleRequest {
    pub registration_number: String,
    pub make: String,
    pub model: String,
    pub year: u16,
    pub simulation: VehicleSimulation,
}