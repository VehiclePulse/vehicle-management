use axum::Router;

use crate::{state::AppState, telemetry, vehicle};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(vehicle::routes::routes())
        .merge(telemetry::routes::routes())
}
