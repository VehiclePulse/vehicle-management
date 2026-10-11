use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Telemetry {
    #[serde(rename = "_id")]
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub vehicle_id: Uuid,

    pub registration_number: String,
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
    #[validate(range(min = 0.0, max = 100.0, message = "front left pressure value is not valid"))]
    pub front_left: f64,

    #[validate(range(min = 0.0, max = 100.0, message = "front right pressure value is not valid"))]
    pub front_right: f64,

    #[validate(range(min = 0.0, max = 100.0, message = "rear left pressure value is not valid"))]
    pub rear_left: f64,

    #[validate(range(min = 0.0, max = 100.0, message = "rear right pressure value is not valid"))]
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

    #[validate(nested)]
    pub tire_pressure: TirePressure,
}

#[cfg(test)] 
mod tests {
    use core::panic;

use super::*;
    use validator::{Validate, ValidationErrorsKind};

    fn valid_update_telemetry_request() -> UpdateTelemetryRequest {
        UpdateTelemetryRequest { 
            latitude: 35.4126, 
            longitude: 85.1420, 
            speed: 120.5, 
            heading: 90.0, 
            fuel_level: 92.5, 
            engine_temperature: 121.0, 
            battery_level: 96.0, 
            tire_pressure: TirePressure { 
                front_left: 78.3, 
                front_right: 73.8, 
                rear_left: 75.4, 
                rear_right: 74.9 
            }
        }
    }

    #[test]
    fn valid_update_telemetry_request_passes_validation() {
        let request = valid_update_telemetry_request();

        assert!(request.validate().is_ok());
    }

    #[test]
    fn latitude_below_minimum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.latitude = -150.0;

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("latitude"));
    }

    #[test]
    fn latitude_above_maximum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.latitude = 750.0;

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("latitude"));
    }

    #[test]
    fn longitude_below_minimum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.longitude = -250.0;

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("longitude"));
    }

    #[test]
    fn longitude_above_maximum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.longitude = 750.0;

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("longitude"));
    }

    #[test]
    fn speed_below_minimum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.speed = -28.6;

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("speed"));
    }

    #[test]
    fn speed_above_maximum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.speed = 728.6;

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("speed"));
    }

    #[test]
    fn heading_below_minimum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.heading = -0.1;

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("heading"));
    }

    #[test]
    fn heading_above_maximium_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.heading = 360.1;

        let errors = request.validate().unwrap_err();

        assert!(errors.field_errors().contains_key("heading"));
    }

    #[test]
    fn fuel_level_below_minimum_fails_valiation() {
        let mut request = valid_update_telemetry_request();

        request.fuel_level = -0.1;

        let errors = request.validate().unwrap_err();

        assert!(
            errors.field_errors().contains_key("fuel_level")
        );
    }

    #[test]
    fn fuel_level_above_maximum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.fuel_level = 111.1;

        let errors = request.validate().unwrap_err();

        assert!(
            errors.field_errors().contains_key("fuel_level")
        )
    }

    #[test]
    fn engine_temperature_below_minimum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.engine_temperature = -500.0;

        let errors = request.validate().unwrap_err();

        assert!(
            errors.field_errors().contains_key("engine_temperature")
        )
    }

    #[test]
    fn engine_temperature_above_maximum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.engine_temperature = 500.0;

        let errors = request.validate().unwrap_err();

        assert!(
            errors.field_errors().contains_key("engine_temperature")
        )
    }
    
    #[test]
    fn battery_level_below_minimum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.battery_level = -0.1;

        let errors = request.validate().unwrap_err();

        assert!(
            errors.field_errors().contains_key("battery_level")
        )
    }

    #[test]
    fn battery_level_above_maximum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.battery_level = -111.1;

        let errors = request.validate().unwrap_err();

        assert!(
            errors.field_errors().contains_key("battery_level")
        )
    }

    #[test]
    fn tire_pressure_front_left_below_minimum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.tire_pressure.front_left = -10.2;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("tire_pressure") {
            Some(ValidationErrorsKind::Struct(tire_pressure_errors)) => {
                assert!(tire_pressure_errors
                    .field_errors()
                    .contains_key("front_left"));
            }

            _ => panic!("expected nested tire pressure validation errors")
        }
    }

    #[test]
    fn tire_pressure_front_left_above_maximum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.tire_pressure.front_left = 1110.2;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("tire_pressure") {
            Some(ValidationErrorsKind::Struct(tire_pressure_errors)) => {
                assert!(tire_pressure_errors
                    .field_errors()
                    .contains_key("front_left"));
            }

            _ => panic!("expected nested tire pressure validation errors")
        }
    }

    #[test]
    fn tire_pressure_front_right_below_minimum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.tire_pressure.front_right = -10.2;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("tire_pressure") {
            Some(ValidationErrorsKind::Struct(tire_pressure_errors)) => {
                assert!(tire_pressure_errors
                    .field_errors()
                    .contains_key("front_right"));
            }

            _ => panic!("expected nested tire pressure validation errors")
        }
    }

    #[test]
    fn tire_pressure_front_right_above_maximum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.tire_pressure.front_right = 120.2;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("tire_pressure") {
            Some(ValidationErrorsKind::Struct(tire_pressure_errors)) => {
                assert!(tire_pressure_errors
                    .field_errors()
                    .contains_key("front_right"));
            }

            _ => panic!("expected nested tire pressure validation errors")
        }
    }

    #[test]
    fn tire_pressure_rear_left_below_minimum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.tire_pressure.rear_left = -10.2;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("tire_pressure") {
            Some(ValidationErrorsKind::Struct(tire_pressure_errors)) => {
                assert!(tire_pressure_errors
                    .field_errors()
                    .contains_key("rear_left"));
            }

            _ => panic!("expected nested tire pressure validation errors")
        }
    }

    #[test]
    fn tire_pressure_rear_left_above_maximum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.tire_pressure.rear_left = 1110.2;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("tire_pressure") {
            Some(ValidationErrorsKind::Struct(tire_pressure_errors)) => {
                assert!(tire_pressure_errors
                    .field_errors()
                    .contains_key("rear_left"));
            }

            _ => panic!("expected nested tire pressure validation errors")
        }
    }

    #[test]
    fn tire_pressure_rear_right_below_minimum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.tire_pressure.rear_right = -10.2;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("tire_pressure") {
            Some(ValidationErrorsKind::Struct(tire_pressure_errors)) => {
                assert!(tire_pressure_errors
                    .field_errors()
                    .contains_key("rear_right"));
            }

            _ => panic!("expected nested tire pressure validation errors")
        }
    }

    #[test]
    fn tire_pressure_rear_right_above_maximum_fails_validation() {
        let mut request = valid_update_telemetry_request();

        request.tire_pressure.rear_right = 120.2;

        let errors = request.validate().unwrap_err();

        match errors.errors().get("tire_pressure") {
            Some(ValidationErrorsKind::Struct(tire_pressure_errors)) => {
                assert!(tire_pressure_errors
                    .field_errors()
                    .contains_key("rear_right"));
            }

            _ => panic!("expected nested tire pressure validation errors")
        }
    }
}