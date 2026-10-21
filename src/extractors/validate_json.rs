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
use serde::{de::DeserializeOwned};
use validator::{
    Validate,
    ValidationErrors
};

use crate::errors::error_details::{ErrorDetails, JsonFieldError};


pub struct ValidateJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidateJson<T>
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
            .map_err(|rejection| {
                let error = ErrorDetails {
                    title: String::from("Invalid Request"),
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    description: rejection.body_text(),
                    errors: None
                };

                (StatusCode::BAD_REQUEST, Json(error)).into_response()

            })?;

        value
            .validate()
            .map_err(|error| {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(ErrorDetails::from(error))
                )
                .into_response()
            })?;

        Ok(Self(value))
    }
}

impl From<ValidationErrors> for ErrorDetails {
    fn from(errors: ValidationErrors) -> Self {
        
        let mut errors_details: HashMap<String, Vec<JsonFieldError>> =
            HashMap::new();

        for (field, validation_errors) in errors.field_errors() {
            let error = validation_errors
                .iter()
                .map(|error| 
                    JsonFieldError {
                        message: error
                            .message
                            .as_ref()
                            .map(|message| message.to_string()),
                        params: error
                            .params
                            .iter()
                            .map(|(key, value)| {
                                (key.to_string(), value.to_string())
                            })
                            .collect()
                    }
                )
                .collect();

            errors_details.insert(field.to_string(), error);
        }

        Self {
            title: String::from("Validation Failed"),
            status_code: 422,
            description: String::from(
                "one or more fields failed validation."
            ),
            errors: Some(errors_details)
        }
    }
}

// #[derive(Serialize)]
// struct JsonValidationError {
//     error: &'static str,
//     details: HashMap<String, Vec<JsonFieldError>>
// }

// #[derive(Debug, Serialize)]
// struct JsonFieldError {
//     code: String,
//     message: Option<String>,
//     params: HashMap<String, String>
// }

// impl From<ValidationErrors> for JsonValidationError {
//     fn from(errors: ValidationErrors) -> Self {
//         let mut details = HashMap::new();

//         for (filed, validation_errors) in errors.field_errors() {
//             let field_errors = validation_errors
//                 .iter()
//                 .map(|validation_error| 
//                     JsonFieldError {
//                         code: validation_error.code.to_string(),
//                         message: validation_error
//                                     .message
//                                     .as_ref()
//                                     .map(|message| message.to_string()),
//                         params: validation_error
//                                     .params
//                                     .iter()
//                                     .map(|(key, value)| (key.to_string(), value.to_string()))
//                                     .collect()
//                     }
//                 )
//                 .collect();

//             details.insert(filed.to_string(), field_errors);
//         };


//         Self {
//             error: "Validation failed",
//             details: details
//         }
//     }
// }