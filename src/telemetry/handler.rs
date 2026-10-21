use crate::{
    error::AppError, extractors::validate_json::ValidateJson, state::AppState, 
    telemetry::{model::{Telemetry, UpdateTelemetryRequest, BatchTelemetryRequest}, validator::validate_manual_update}
};

use axum::{
    Json, extract::{Path, State},
};

pub async fn get_vehicles_telemetry(
    State(state): State<AppState>,
    Json(body): Json<BatchTelemetryRequest>
) -> Result<Json<Vec<Telemetry>>, AppError>  {
    if body.registration_numbers.is_empty() {
        return Err(AppError::Business(
            "Vehicle id's cannot be empty".to_string()
        ));
    } 

    match state
        .telemetry_repository
        .get_telemetry_for_multiple_vehicles(body.registration_numbers)
        .await {
            Ok(telemetry_list) => Ok(Json(telemetry_list)),
            Err(err) => Err(AppError::Database(err))
        }
}

pub async fn get_vehicle_telemetry(
    State(state): State<AppState>,
    Path(registration_number): Path<String>,
) -> Result<Json<Telemetry>, AppError> {
    tracing::debug!(
        registration_number = %registration_number,
        "retrieving vehicle telemetry"
    );

    let telemetry = state
        .telemetry_repository
        .get_by_vehicle_id(registration_number.clone())
        .await
        .map_err(|error| {
            tracing::error!(
                vehicle_id = %registration_number,
                error = %error,
                "failed to retrieve telemetry"
            );

            AppError::Database(error)
        }
    )?;

    tracing::info!(
        vehicle_id = %registration_number,
        "retrieved telemetry record"
    );

    match telemetry {
        Some(telemetry) => Ok(Json(telemetry)),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_vehicle_telemetry(
    State(state): State<AppState>,
    Path(registration_number): Path<String>,
    ValidateJson(request): ValidateJson<UpdateTelemetryRequest>,
) -> Result<Json<Telemetry>, AppError> {
    tracing::debug!(
        registration_number= %registration_number,
        "processing telemetry update"
    );

    let vehicle = state
        .vehicle_repository
        .get_by_registration_number(registration_number.clone())
        .await
        .map_err(|error| {
            tracing::error!(
                registration_number = %registration_number,
                error = %error,
                "failed to retrieve vehicle"
            );

            AppError::Database(error)
        })?;

    let vehicle = match vehicle {
        Some(vehicle) => vehicle,
        None => {
            tracing::warn!(
                registration_number = %registration_number,
                "telemetry update rejected: vehicle not found"
            );

            return Err(AppError::NotFound)
        } 
    };

    validate_manual_update(&vehicle)?;

    let telemetry = Telemetry {
        vehicle_id: vehicle.id,
        registration_number: vehicle.registration_number,
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
                vehicle_id = %registration_number,
                error = %error,
                "failed to save telemetry"
            );

            AppError::Database(error)
        })?;

    tracing::info!(
        vehicle_id = %registration_number,
        "vehicle telemetry updated"
    );

    Ok(Json(telemetry))
}
