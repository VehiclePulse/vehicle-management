use axum::Json;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, header};
use axum::http::{StatusCode, HeaderValue};
use axum::response::{IntoResponse, Response};

use crate::errors::error_types::{AppErrorKind, DomainError};
use crate::extractors::validate_json::ValidateJson;
use crate::extractors::validate_query::ValidateQuery;
use crate::state::AppState;
use crate::vehicle::model::{Vehicle, VehicleRequest, GetActiveSimulation};

pub async fn get_vehicles( 
    State(state): State<AppState>,
    ValidateQuery(params): ValidateQuery<GetActiveSimulation>
) -> Result<Response, AppErrorKind> {
    let vehicles = state
        .vehicle_service
        .get_all(params)
        .await?;

    if vehicles.is_empty() {
        tracing::info!(
            "retrieved zero vehicles details"
        );
        
        return Ok(StatusCode::NO_CONTENT.into_response());
    }

    tracing::info!(
        "retrieved all vehicles details"
    );

    Ok(Json(vehicles).into_response())
}

pub async fn create_vehicle(
    State(state): State<AppState>,
    ValidateJson(request): ValidateJson<VehicleRequest>,
) -> Result<Response, AppErrorKind> {
    let vehicle = Vehicle {
        id: uuid::Uuid::new_v4(),
        registration_number: request.registration_number,
        make: request.make,
        model: request.model,
        year: request.year,
        simulation: request.simulation,
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::LOCATION,
        HeaderValue::from_str(&format!(
            "/vehicles/{}",
            vehicle.registration_number
        ))
        .map_err(|_| {
            AppErrorKind::InvalidRouteValue
        })?,
    );

    println!("after wrong input");

    state
        .vehicle_service
        .create(vehicle.clone())
        .await?;

    println!("inserted vehicle");

    tracing::info!(
        vehicle_id = %vehicle.id,
        "vehicle record created successfully"
    );

    Ok(
        (StatusCode::CREATED, headers, Json(vehicle))
        .into_response()
    )
}

pub async fn get_vehicle(
    State(state): State<AppState>,
    Path(registration_number): Path<String>,
) -> Result<Response, AppErrorKind> {
    let vehicle = state
        .vehicle_service
        .get_by_registration_number(registration_number.clone())
        .await?;

    tracing::info!(
        registration_number = %registration_number,
        "retrieving vehicle record"
    );

    Ok((
        StatusCode::OK,
        Json(vehicle)
    ).into_response()
    )
}

pub async fn update_vehicle(
    State(state): State<AppState>,
    Path(registration_number): Path<String>,
    Json(request): Json<VehicleRequest>,
) -> Result<Response, AppErrorKind> {
    tracing::info!(
        vehicle_id = %registration_number,
        "updating vehicle record"
    );

    let existing_vehicle = state.vehicle_service
        .get_by_registration_number(registration_number.clone())
        .await?;

    let vehicle = Vehicle {
        id: existing_vehicle.id,
        registration_number: request.registration_number,
        make: request.make,
        model: request.model,
        year: request.year,
        simulation: request.simulation,
    };

    let updated = state
        .vehicle_service
        .update(registration_number.clone(), vehicle.clone())
        .await?;

    if !updated {
        return Err(DomainError::VehicleNotFound.into());
    }

    Ok(StatusCode::NO_CONTENT.into_response())
}

pub async fn delete_vehicle(
    State(state): State<AppState>,
    Path(registration_number): Path<String>,
) -> Result<Response, AppErrorKind> {
    tracing::info!(
        vehicle_id = %registration_number,
        "deleting vehicle record"
    );

    let status = state
        .vehicle_service
        .delete(registration_number.clone())
        .await?;

    if !status {
        return Err(DomainError::VehicleNotFound.into());
    }

    Ok(StatusCode::NO_CONTENT.into_response())
}
