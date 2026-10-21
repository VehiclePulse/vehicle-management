use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,

    #[error("Database operation failed")]
    Database(mongodb::error::Error),

    #[error("{0}")]
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
                    error.to_string(),
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