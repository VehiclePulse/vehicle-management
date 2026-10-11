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
                    "Database failure".to_string(),
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


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn app_errors_not_found_returns_404_and_message() {
        let error = AppError::NotFound;

        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();

        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], "Vehicle not found");
    }

    #[tokio::test]
    async fn app_errors_business_error_return_422_and_message() {
        let error = AppError::Business(
            "Telemetry cannot be manually updated for a simulated vehicle".to_string()  
        );

        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();

        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body["error"],
            "Telemetry cannot be manually updated for a simulated vehicle"
        );
    }

    #[tokio::test]
    async fn app_errors_database_error_return_500_and_message() {
        let database_error = mongodb::error::Error::custom("test database error");

        let error = AppError::Database(database_error);

        let response = error.into_response();

        assert_eq!(
            response.status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)   
            .await
            .unwrap();

        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body["error"], "Internal server error"
        );
    }
}