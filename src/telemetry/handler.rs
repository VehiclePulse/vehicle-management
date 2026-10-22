use crate::{
    errors::error_types::{AppErrorKind, DomainError}, extractors::validate_json::ValidateJson, state::AppState, telemetry::{model::{BatchTelemetryRequest, Telemetry, UpdateTelemetryRequest}, validator::validate_manual_update}
};

use axum::{
    Json, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response},
};

pub async fn get_vehicles_telemetry(
    State(state): State<AppState>,
    ValidateJson(body): ValidateJson<BatchTelemetryRequest>
) -> Result<Response, AppErrorKind>  {
    if body.registration_numbers.is_empty() {
        return Err(AppErrorKind::Domain(
            DomainError::TelemetryBatchEmpty
        ));
    } 

    let telemeries = state
        .telemetry_service
        .get_telemetry_for_multiple_vehicles(body.registration_numbers)
        .await?;

    if telemeries.is_empty() {
        return Ok(StatusCode::NO_CONTENT.into_response());
    }

    return Ok(
        Json(telemeries).into_response()
    )
}

pub async fn get_vehicle_telemetry(
    State(state): State<AppState>,
    Path(registration_number): Path<String>,
) -> Result<Response, AppErrorKind> {
    tracing::debug!(
        registration_number = %registration_number,
        "retrieving vehicle telemetry"
    );

    let telemetry = state
        .telemetry_service
        .get_by_vehicle_id(registration_number)
        .await?;

    Ok((
        StatusCode::OK, 
        Json(telemetry)
    ).into_response())
}

pub async fn update_vehicle_telemetry(
    State(state): State<AppState>,
    Path(registration_number): Path<String>,
    ValidateJson(request): ValidateJson<UpdateTelemetryRequest>,
) -> Result<Response, AppErrorKind> {
    tracing::debug!(
        registration_number= %registration_number,
        "processing telemetry update"
    );

    let vehicle = state
        .vehicle_service
        .get_by_registration_number(registration_number.clone())
        .await?;

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
        .telemetry_service
        .upsert(telemetry.clone())
        .await?;

    tracing::info!(
        vehicle_id = %registration_number,
        "vehicle telemetry updated"
    );

    Ok(Json(telemetry).into_response())
}
