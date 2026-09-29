use crate::{
    error::AppError,
    vehicle::model::Vehicle
};

pub fn validate_manual_update(vehicle: &Vehicle) -> Result<(), AppError> {
    if vehicle.simulation.enabled {
        return Err(AppError::Business(
            "Telemetry manually cannot be updated for a simulated vehicle".to_string()
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vehicle::model::{Vehicle, VehicleSimulation};
    use uuid::Uuid;

    fn create_vehicle(simulation_enabled: bool) -> Vehicle {
        Vehicle {
            id: Uuid::new_v4(),
            registration_number: "KA 01 AB 1001".to_string(),
            make: "Toyota".to_string(),
            model: "Camry".to_string(),
            year: 2024,
            simulation: VehicleSimulation { 
                enabled: simulation_enabled, 
                initial_latitude: 12.9716, 
                initial_longitude: 77.5946, 
                initial_heading: 90.0 
            }
        }
    }

    #[test]
    fn allows_manual_telemetry_update_for_non_simulated_vehicle() {
        let vehicle = create_vehicle(false);

        let result = validate_manual_update(&vehicle);

        assert!(result.is_ok());
    }

    #[test]
    fn reject_manual_telemetry_update_for_simulated_vehicle() {
        let vehicle = create_vehicle(true);

        let result = validate_manual_update(&vehicle);

        assert!(result.is_err());


        match result {
            Err(AppError::Business(message)) => {
                assert_eq!(
                    message,
                    "Telemetry manually cannot be updated for a simulated vehicle"
                );
            }

            _ => panic!("Expected AppError::Business")
        }
    }
}