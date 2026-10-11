use axum::{Router, routing::get};

use crate::state::AppState;
use crate::vehicle::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/vehicles",
            get(handler::get_vehicles).post(handler::create_vehicle),
        )
        .route(
            "/vehicles/{registration_number}",
            get(handler::get_vehicle)
                .put(handler::update_vehicle)
                .delete(handler::delete_vehicle),
        )
}
