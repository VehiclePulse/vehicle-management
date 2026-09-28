use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Database(mongodb::error::Error),
    Business(String),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Vehicle not found".to_string()),

            AppError::Database(error) => {
                tracing::error!("Database error: {}", error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            },

            AppError::Business(error) => {
                tracing::error!("Business error: {}", error);
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    error.to_string(),
                )
            }
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}
