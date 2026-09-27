use axum::{Router, routing::get};

use crate::state::AppState;
use crate::telemetry::handler;

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/vehicles/{id}/telemetry",
        get(handler::get_vehicle_telemetry).put(handler::update_vehicle_telemetry),
    )
}
