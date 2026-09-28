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