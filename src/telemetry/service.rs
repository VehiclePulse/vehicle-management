use crate::{errors::error_types::{AppErrorKind, DomainError, InfraError}, telemetry::{model::Telemetry, repository::TelemetryRepository}};

#[derive(Clone)]
pub struct TelemetryService {
    pub repository: TelemetryRepository
}

impl TelemetryService {
    pub fn new(repository: TelemetryRepository) -> Self {
        TelemetryService { repository }
    }

    pub async fn get_telemetry_for_multiple_vehicles(
        &self,
        registration_numbers: Vec<String>
    ) -> Result<Vec<Telemetry>, AppErrorKind> {
        self
        .repository
        .get_telemetry_for_multiple_vehicles(registration_numbers)
        .await
        .map_err(|error| {
            AppErrorKind::Infrastructure(
                InfraError::Database(
                    anyhow::Error::new(error)
                    .context("Failed to retrive the telemetries")
                )
            )
        })
    }

    pub async fn get_by_vehicle_id(
        &self,
        registration_number: String,
    ) -> Result<Telemetry, AppErrorKind> {
        let telemetry = self
            .repository
            .get_by_vehicle_id(registration_number)
            .await
            .map_err(|error| {
                AppErrorKind::Infrastructure(
                    InfraError::Database(
                        anyhow::Error::new(error)
                        .context("Failed to retieve telemetry")
                    )
                )
            })?;

        telemetry.ok_or(AppErrorKind::Domain(
            DomainError::TelemetryNotFound
        ))
    }

    pub async fn upsert(
        &self,
        telemetry: Telemetry
    ) -> Result<(), AppErrorKind> {
        self
            .repository
            .upsert(telemetry)
            .await
            .map_err(|error| {
                AppErrorKind::Infrastructure(
                    InfraError::Database(
                        anyhow::Error::new(error)
                        .context("Failed to upsert the telemetry")
                    )
                )
            })
    }

}