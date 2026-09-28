use crate::{
    error::AppError, 
    extractors::validated_json::ValidatedJson, 
    state::AppState, 
    telemetry::model::{Telemetry, UpdateTelemetryRequest},
    telemetry::validator::validate_manual_update
};

use axum::{
    Json,
    extract::{Path, State},
};

pub async fn get_vehicle_telemetry(
    State(state): State<AppState>,
    Path(vehicle_id): Path<uuid::Uuid>,
) -> Result<Json<Telemetry>, AppError> {
    tracing::debug!(
        vehicle_id = %vehicle_id,
        "retrieving vehicle telemetry"
    );

    let telemetry = state
        .telemetry_repository
        .get_by_vehicle_id(vehicle_id)
        .await
        .map_err(|error| {
            tracing::error!(
                vehicle_id = %vehicle_id,
                error = %error,
                "failed to retrieve telemetry"
            );

            AppError::Database(error)
        }
    )?;

    tracing::info!(
        vehicle_id = %vehicle_id,
        "retrieved telemetry record"
    );

    match telemetry {
        Some(telemetry) => Ok(Json(telemetry)),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_vehicle_telemetry(
    State(state): State<AppState>,
    Path(vehicle_id): Path<uuid::Uuid>,
    ValidatedJson(request): ValidatedJson<UpdateTelemetryRequest>,
) -> Result<Json<Telemetry>, AppError> {
    tracing::debug!(
        vehicle_id= %vehicle_id,
        "processing telemetry update"
    );

    let vehicle = state
        .vehicle_repository
        .get_by_id(vehicle_id)
        .await
        .map_err(|error| {
            tracing::error!(
                vehicle_id = %vehicle_id,
                error = %error,
                "failed to retrieve vehicle"
            );

            AppError::Database(error)
        })?;

    let vehicle = match vehicle {
        Some(vehicle) => vehicle,
        None => {
            tracing::warn!(
                vehicle_id = %vehicle_id,
                "telemetry update rejected: vehicle not found"
            );

            return Err(AppError::NotFound)
        } 
    };

    validate_manual_update(&vehicle)?;

    let telemetry = Telemetry {
        vehicle_id,
        latitude: request.latitude,
        longitude: request.longitude,
        speed: request.speed,
        heading: request.heading,
        fuel_level: request.fuel_level,
        engine_temperature: request.engine_temperature,
        battery_level: request.battery_level,
        tire_pressure: request.tire_pressure,
        last_updated_at: chrono::Utc::now(),
    };

    state
        .telemetry_repository
        .upsert(telemetry.clone())
        .await
        .map_err(|error| {
            tracing::error!(
                vehicle_id = %vehicle_id,
                error = %error,
                "failed to save telemetry"
            );

            AppError::Database(error)
        })?;

    tracing::info!(
        vehicle_id = %vehicle_id,
        "vehicle telemetry updated"
    );

    Ok(Json(telemetry))
}
