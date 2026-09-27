use std::collections::HashMap;

use axum::{
    extract::{
        FromRequest, Json, Request
    },
    response::{
        IntoResponse, Response
    },
    http::StatusCode
};
use serde::{Serialize, de::DeserializeOwned};
use validator::{
    Validate,
    ValidationErrors
};


pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate
{
    type Rejection = Response;

    async fn from_request(
        request: Request,
        state: &S
    ) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(request, state)
            .await
            .map_err(IntoResponse::into_response)?;

        value
            .validate()
            .map_err(|error| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(JsonValidationError::from(error))
                )
                .into_response()
            })?;

        Ok(Self(value))
    }
}

#[derive(Serialize)]
struct JsonValidationError {
    error: &'static str,
    details: HashMap<String, Vec<JsonFieldError>>
}

#[derive(Debug, Serialize)]
struct JsonFieldError {
    code: String,
    message: Option<String>,
    params: HashMap<String, String>
}

impl From<ValidationErrors> for JsonValidationError {
    fn from(errors: ValidationErrors) -> Self {
        let mut details = HashMap::new();

        for (filed, validation_errors) in errors.field_errors() {
            let field_errors = validation_errors
                .iter()
                .map(|validation_error| 
                    JsonFieldError {
                        code: validation_error.code.to_string(),
                        message: validation_error
                                    .message
                                    .as_ref()
                                    .map(|message| message.to_string()),
                        params: validation_error
                                    .params
                                    .iter()
                                    .map(|(key, value)| (key.to_string(), value.to_string()))
                                    .collect()
                    }
                )
                .collect();

            details.insert(filed.to_string(), field_errors);
        };


        Self {
            error: "Validation failed",
            details: details
        }
    }
}