use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::extractors::validated_json::ValidatedJson;
use crate::error::AppError;
use crate::state::AppState;
use crate::vehicle::model::{Vehicle, CreateVehicleRequest, UpdateVehicleRequest};

pub async fn get_vehicles(State(state): State<AppState>) -> Result<Json<Vec<Vehicle>>, AppError> {
    let vehicles = state
        .vehicle_repository
        .get_all()
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
    ValidatedJson(request): ValidatedJson<CreateVehicleRequest>,
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
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Vehicle>, AppError> {
    let vehicle = state
        .vehicle_repository
        .get_by_id(id)
        .await
        .map_err(|error| {
            tracing::error!(
                vehicle_id = %id,
                error = %error,
                "retrieving vehicle failed"
            );

            AppError::Database(error)
        })?;

    tracing::info!(
        vehicle_id = %id,
        "retrieving vehicle record"
    );

    match vehicle {
        Some(vehicle) => Ok(Json(vehicle)),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_vehicle(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(request): Json<UpdateVehicleRequest>,
) -> Result<Json<Vehicle>, AppError> {
    let vehicle = Vehicle {
        id,
        registration_number: request.registration_number,
        make: request.make,
        model: request.model,
        year: request.year,
        simulation: request.simulation,
    };

    let updated = state
        .vehicle_repository
        .update(id, vehicle.clone())
        .await
        .map_err(|error| {
            tracing::error!(
                vehicle_id = %id,
                error = %error,
                "failed to update vehicle record"
            );

            AppError::Database(error)
        })?;

    tracing::info!(
        vehicle_id = %id,
        "updating vehicle record"
    );

    if !updated {
        return Err(AppError::NotFound);
    }

    Ok(Json(vehicle))
}

pub async fn delete_vehicle(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, AppError> {
    let deleted = state
        .vehicle_repository
        .delete(id)
        .await
        .map_err(|error| {
            tracing::error!(
                vehicle_id = %id,
                error = %error,
                "deleting vehicle failed"
            );

            AppError::Database(error)
        })?;

    tracing::info!(
        vehicle_id = %id,
        "deleting vehicle record"
    );

    if !deleted {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
