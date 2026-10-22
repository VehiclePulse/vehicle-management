use crate::{
    errors::error_types::{AppErrorKind, DomainError}, vehicle::model::Vehicle
};

pub fn validate_manual_update(vehicle: &Vehicle) -> Result<(), AppErrorKind> {
    if vehicle.simulation {
        return Err(AppErrorKind::Domain(
            DomainError::VehicleTelemetryUpdateFailed
        ));
    }

    Ok(())
}