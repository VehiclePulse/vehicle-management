use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::extractors::validate_json::ValidateJson;
use crate::extractors::validate_query::ValidateQuery;
use crate::error::AppError;
use crate::errors::error_details::ErrorDetails;
use crate::state::AppState;
use crate::vehicle::model::{Vehicle, VehicleRequest, GetActiveSimulation};

pub async fn get_vehicles( 
    State(state): State<AppState>,
    ValidateQuery(params): ValidateQuery<GetActiveSimulation>
) -> Result<Json<Vec<Vehicle>>, AppError> {
    let vehicles = state
        .vehicle_repository
        .get_all(params)
        .await
        .map_err(|error| {
            tracing::error!(
                error = %error,
                "failed retrieving all vehicles details"
            );

            AppError::Database(error)
        })?;

    tracing::info!(
        "retrieved all vehicles details"
    );

    Ok(Json(vehicles))
}

pub async fn create_vehicle(
    State(state): State<AppState>,
    ValidateJson(request): ValidateJson<VehicleRequest>,
) -> Result<(StatusCode, Json<Vehicle>), AppError> {
    let vehicle = Vehicle {
        id: uuid::Uuid::new_v4(),
        registration_number: request.registration_number,
        make: request.make,
        model: request.model,
        year: request.year,
        simulation: request.simulation,
    };

    state
        .vehicle_repository
        .create(vehicle.clone())
        .await
        .map_err(|error| {
            tracing::error!(
                vehicle_id = %vehicle.id,
                error = %error,
                "vehicle saving failed"
            );

            AppError::Database(error)
        })?;

    tracing::info!(
        vehicle_id = %vehicle.id,
        "vehicle record created successfully"
    );

    Ok((StatusCode::CREATED, Json(vehicle)))
}

pub async fn get_vehicle(
    State(state): State<AppState>,
    Path(registration_number): Path<String>,
) -> Result<Json<Vehicle>, AppError> {
    let vehicle = state
        .vehicle_repository
        .get_by_registration_number(registration_number.clone())
        .await
        .map_err(|error| {
            tracing::error!(
                registration_number = %registration_number,
                error = %error,
                "retrieving vehicle failed"
            );

            AppError::Database(error)
        })?;

    tracing::info!(
        registration_number = %registration_number,
        "retrieving vehicle record"
    );

    match vehicle {
        Some(vehicle) => Ok(Json(vehicle)),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_vehicle(
    State(state): State<AppState>,
    Path(registration_number): Path<String>,
    Json(request): Json<VehicleRequest>,
) -> Result<Json<Vehicle>, AppError> {
    
    let existing_vehicle = state.vehicle_repository.get_by_registration_number(registration_number.clone()).await.
        map_err(|error| {
                tracing::error!(
                    registration_number = %registration_number,
                    error = %error,
                    "retrieving vehicle failed"
                );

                AppError::Database(error)
            })?;

    let Some(existing) = existing_vehicle else {
        tracing::error!(
            registration_number = %registration_number,
            "vehicle not found"
        );
        return Err(AppError::NotFound);
    };

    let vehicle = Vehicle {
        id: existing.id,
        registration_number: request.registration_number,
        make: request.make,
        model: request.model,
        year: request.year,
        simulation: request.simulation,
    };

    let updated = state
        .vehicle_repository
        .update(registration_number.clone(), vehicle.clone())
        .await
        .map_err(|error| {
            tracing::error!(
                vehicle_id = %registration_number,
                error = %error,
                "failed to update vehicle record"
            );

            AppError::Database(error)
        })?;

    tracing::info!(
        vehicle_id = %registration_number,
        "updating vehicle record"
    );

    if !updated {
        return Err(AppError::NotFound);
    }

    Ok(Json(vehicle))
}

pub async fn delete_vehicle(
    State(state): State<AppState>,
    Path(registration_number): Path<String>,
) -> Result<StatusCode, AppError> {
    let deleted = state
        .vehicle_repository
        .delete(registration_number.clone())
        .await
        .map_err(|error| {
            tracing::error!(
                vehicle_id = %registration_number,
                error = %error,
                "deleting vehicle failed"
            );

            AppError::Database(error)
        })?;

    tracing::info!(
        vehicle_id = %registration_number,
        "deleting vehicle record"
    );

    if !deleted {
        tracing::error!(
            vehicle_id = %registration_number,
            "error deleting vehicle"
        );
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
