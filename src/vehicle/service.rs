use mongodb::error::{
    Error,
    ErrorKind,
    WriteFailure
};

use crate::{
    errors::error_types::{AppErrorKind, DomainError, InfraError}, vehicle::{model::Vehicle, repository::VehicleRepository}
};
use crate::vehicle::model::{GetActiveSimulation};

#[derive(Clone)]
pub struct VehicleService {
    repository: VehicleRepository
}

impl VehicleService {
    pub fn new(repository: VehicleRepository) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self, 
        vehicle: Vehicle
    ) -> Result<(), AppErrorKind> {
        self.repository
            .create(vehicle.clone())
            .await
            .map_err(|error| {
                if is_duplicate_key_error(&error) {
                    return AppErrorKind::Domain(DomainError::VehicleAlreadyExists);
                }

                AppErrorKind::Infrastructure(
                    InfraError::Database(
                        anyhow::Error::new(error)
                            .context("failed to save vehicle to database")
                    )  
                )
            })?;
            
            Ok(())
    }

    pub async fn get_by_registration_number(
        &self,
        registration_number: String
    ) -> Result<Vehicle, AppErrorKind> {
        let vehicle = self
            .repository
            .get_by_registration_number(registration_number)
            .await
            .map_err(|error| {
                AppErrorKind::Infrastructure(
                    InfraError::Database(
                        anyhow::Error::new(error)
                        .context("Failed to retrieve vehicle")
                    )
                )
            })?;

        vehicle.ok_or(AppErrorKind::Domain(
            DomainError::VehicleNotFound
        ))   
    }

    pub async fn get_all(
        &self,
        query: GetActiveSimulation
    ) -> Result<Vec<Vehicle>, AppErrorKind> {
        let vehicles = self
            .repository
            .get_all(query)
            .await
            .map_err(|error| {
                AppErrorKind::Infrastructure(
                    InfraError::Database(
                        anyhow::Error::new(error)
                        .context("Failed to retrieve vehicle")
                    )
                )
            })?;

        Ok(vehicles)
    }

    pub async fn update(
        &self,
        registration_number: String, 
        vehicle: Vehicle        
    ) -> Result<bool, AppErrorKind> {
        let updated = self
            .repository
            .update(
                registration_number, 
                vehicle
            )
            .await
            .map_err(|error| {
                AppErrorKind::Infrastructure(
                    InfraError::Database(
                        anyhow::Error::new(error)
                        .context("Failed to update vehicle")
                ))
            })?;

        Ok(updated)
    }

    pub async fn delete(
        &self,
        registration_number: String
    ) -> Result<bool, AppErrorKind> {
        let deleted = self
            .repository
            .delete(registration_number)
            .await
            .map_err(|error| {
                AppErrorKind::Infrastructure(
                    InfraError::Database(
                        anyhow::Error::new(error)
                        .context("Failed to delete vehicle")
                    )
                )
            })?;

        Ok(deleted)
    }
}


fn is_duplicate_key_error(error: &Error) -> bool {
    matches!(
        error.kind.as_ref(),
        ErrorKind::Write(WriteFailure::WriteError(write_error))
            if write_error.code == 11000
    )
}