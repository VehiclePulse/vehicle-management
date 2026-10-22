use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

use crate::errors::error_details::ErrorDetails;

#[derive(Debug, Error)]
pub enum AppErrorKind {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Infrastructure(#[from] InfraError),

    #[error("Failed to construct HTTP response header")]
    InvalidRouteValue
}


#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Vehcile doesn't exist")]
    VehicleNotFound,

    #[error("Vehicle with the registration number already exists")]
    VehicleAlreadyExists,

    #[error("Telemetry manually cannot be updated for a simulated vehicle")]
    VehicleTelemetryUpdateFailed,

    #[error("Telemetry batch request cannot be empty")]
    TelemetryBatchEmpty,

    #[error("Telemetry data not found")]
    TelemetryNotFound
}


#[derive(Debug, Error)]
pub enum InfraError {
    #[error("Database operation failed")]
    Database(#[source] anyhow::Error)
}

impl IntoResponse for AppErrorKind {
    fn into_response(self) -> Response {
        let (etitle, estatus, edescription): (String, StatusCode, String) = match self {
            AppErrorKind::Domain(error) => {
                let status = match &error {
                    DomainError::VehicleNotFound |
                    DomainError::TelemetryNotFound => StatusCode::NOT_FOUND,
                    DomainError::VehicleAlreadyExists => StatusCode::CONFLICT,
                    DomainError::VehicleTelemetryUpdateFailed => StatusCode::CONFLICT,
                    DomainError::TelemetryBatchEmpty => StatusCode::UNPROCESSABLE_ENTITY
                };
                (
                    error.to_string(),
                    status, 
                    match &error {
                        DomainError::VehicleAlreadyExists => 
                            String::from("Vehicle with the registration number already exists"),
                        DomainError::VehicleNotFound => 
                            String::from("Vehicle doesn't exist"),
                        DomainError::VehicleTelemetryUpdateFailed => 
                            String::from("Telemetry manually cannot be updated for a simulated vehicle") ,
                        DomainError::TelemetryBatchEmpty =>
                            String::from("Telemetry batch request cannot be empty"),
                        DomainError::TelemetryNotFound => 
                            String::from("Telemetry data not found")
                    }
                )
            }

            AppErrorKind::Infrastructure(error) => {
                let title = match &error {
                    InfraError::Database(source) => source.to_string(),
                };

                (
                    title,
                    StatusCode::INTERNAL_SERVER_ERROR,
                    error.to_string()
                )
            }

            AppErrorKind::InvalidRouteValue => {
                (
                    String::from("Invalid value"),
                    StatusCode::BAD_REQUEST,
                    String::from("Path value is not valid")
                )
            }
        }; 

        (   
            estatus,
            Json(ErrorDetails {
                title: etitle,
                status_code: estatus.as_u16(),
                description: edescription.to_string(),
                errors: None
            })
        ).into_response()
    }
}