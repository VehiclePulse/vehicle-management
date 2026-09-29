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

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct VehicleSimulation {
    pub enabled: bool,

    #[validate(range(min = -90.0, max = 90.0, message = "latitude value is not valid"))]
    pub initial_latitude: f64,

    #[validate(range(min = -180.0, max = 180.0, message = "longitude value is not valid"))]
    pub initial_longitude: f64,

    #[validate(range(min = 0.0, max = 360.0, message = "initial heading value is not valid"))]
    pub initial_heading: f64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VehicleRequest {

    #[validate(length(min = 1, max = 14, message = "registration number doesn't satisfy defined length"))]
    pub registration_number: String,

    #[validate(length(min = 1, max = 20, message = "make doesn't satisfy defined length"))]
    pub make: String,

    #[validate(length(min = 1, max = 20, message = "model doesn't satisfy defined length"))]
    pub model: String,

    #[validate(range(min = 1990, max = 2100, message = "year doesn't satisfy defined range"))]
    pub year: u16,

    #[validate(nested)]
    pub simulation: VehicleSimulation,
}


#[cfg(test)]
mod tests {
    use super::*;
    use validator::{Validate, ValidationErrorsKind};

    fn valid_request() -> VehicleRequest {
        VehicleRequest {
            registration_number: "KA01AB1234".to_string(),
            make: "Toyota".to_string(),
            model: "Camry".to_string(),
            year: 2024,
            simulation: VehicleSimulation {
                enabled: false,
                initial_latitude: 12.9716,
                initial_longitude: 77.5946,
                initial_heading: 90.0,
            },
        }
    }

    #[test]
    fn valid_create_vehicle_request_passes_validation() {
        let request = valid_request();

        assert!(request.validate().is_ok());
    }

    #[test]
    fn registration_number_over_max_length_is_rejected() {
        let mut request = valid_request();

        request.registration_number = String::from("123456789012345");

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("registration_number"));
    }

    #[test]
    fn registration_number_empty_is_rejected() {
        let mut request = valid_request();

        request.registration_number.clear();

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("registration_number"));
    }

    #[test]
    fn make_over_max_length_is_rejected() {
        let mut request = valid_request();

        request.make = "1234567890123456789012".to_string();

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("make"));
    }

    #[test]
    fn model_over_max_length_is_rejected() {
        let mut request = valid_request();

        request.model = "123456789012345678901".to_string();

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("model"));
    }

    #[test]
    fn year_below_minimum_is_rejected() {
        let mut request = valid_request();

        request.year = 1989;

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("year"));
    }

    #[test]
    fn year_above_maximum_is_rejected() {
        let mut request = valid_request();

        request.year = 2101;

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("year"));
    }

    #[test]
    fn latitude_below_minimum_is_rejected() {
        let mut request = valid_request();

        request.simulation.initial_latitude = -91.0;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("simulation") {
            Some(ValidationErrorsKind::Struct(simulation_errors)) => {
                assert!(simulation_errors
                    .field_errors()
                    .contains_key("initial_latitude"));
            }
            _ => panic!("expected nested simulation validation errors"),
        }
    }

    #[test]
    fn latitude_above_maximum_is_rejected() {
        let mut request = valid_request();

        request.simulation.initial_latitude = 91.0;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("simulation") {
            Some(ValidationErrorsKind::Struct(simulation_errors)) => {
                assert!(simulation_errors
                    .field_errors()
                    .contains_key("initial_latitude"));
            }
            _ => panic!("expected nested simulation validation errors"),
        }
    }

    #[test]
    fn longitude_below_minimum_is_rejected() {
        let mut request = valid_request();

        request.simulation.initial_longitude = -180.1;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("simulation") {
            Some(ValidationErrorsKind::Struct(simulation_errors)) => {
                assert!(simulation_errors
                    .field_errors()
                    .contains_key("initial_longitude"));
            }
            _ => panic!("expected nested simulation validation errors"),
        }
    }

    #[test]
    fn longitude_above_maximum_is_rejected() {
        let mut request = valid_request();

        request.simulation.initial_longitude = 180.1;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("simulation") {
            Some(ValidationErrorsKind::Struct(simulation_errors)) => {
                assert!(simulation_errors
                    .field_errors()
                    .contains_key("initial_longitude"));
            }
            _ => panic!("expected nested simulation validation errors"),
        }
    }

    #[test]
    fn heading_below_minimum_is_rejected() {
        let mut request = valid_request();

        request.simulation.initial_heading = -1.0;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("simulation") {
            Some(ValidationErrorsKind::Struct(simulation_errors)) => {
                assert!(simulation_errors
                    .field_errors()
                    .contains_key("initial_heading"));
            }
            _ => panic!("expected nested simulation validation errors"),
        }
    }

    #[test]
    fn heading_above_maximum_is_rejected() {
        let mut request = valid_request();

        request.simulation.initial_heading = 361.0;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("simulation") {
            Some(ValidationErrorsKind::Struct(simulation_errors)) => {
                assert!(simulation_errors
                    .field_errors()
                    .contains_key("initial_heading"));
            }
            _ => panic!("expected nested simulation validation errors"),
        }
    }
}