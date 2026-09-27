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
        .map_err(AppError::Database)?;

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
        .map_err(AppError::Database)?;

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
        .map_err(AppError::Database)?;

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
        .map_err(AppError::Database)?;

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
        .map_err(AppError::Database)?;

    if !deleted {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
