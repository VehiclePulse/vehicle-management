use crate::{
    error::AppError,
    state::AppState,
    telemetry::model::{Telemetry, TirePressure},
};

use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;

pub async fn get_vehicle_telemetry(
    State(state): State<AppState>,
    Path(vehicle_id): Path<uuid::Uuid>,
) -> Result<Json<Telemetry>, AppError> {
    let telemetry = state
        .telemetry_repository
        .get_by_vehicle_id(vehicle_id)
        .await
        .map_err(AppError::Database)?;

    match telemetry {
        Some(telemetry) => Ok(Json(telemetry)),
        None => Err(AppError::NotFound),
    }
}

#[derive(Deserialize, Clone)]
pub struct UpdateTelemetryRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub heading: f64,

    pub fuel_level: f64,
    pub engine_temperature: f64,
    pub battery_level: f64,

    pub tire_pressure: TirePressure,
}

pub async fn update_vehicle_telemetry(
    State(state): State<AppState>,
    Path(vehicle_id): Path<uuid::Uuid>,
    Json(request): Json<UpdateTelemetryRequest>,
) -> Result<Json<Telemetry>, AppError> {
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
        .map_err(AppError::Database)?;

    Ok(Json(telemetry))
}
