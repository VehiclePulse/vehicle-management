use axum::{
    extract::{
        FromRequestParts,
        Query,
        rejection::QueryRejection
    },
    http::{
        request::Parts,
        StatusCode
    },
    response::{
        IntoResponse,
        Response
    },
    Json
};
use serde::de::DeserializeOwned;

use crate::errors::error_details::ErrorDetails;

pub struct ValidateQuery<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidateQuery<T>
where 
    S: Send + Sync,
    T: DeserializeOwned
{
    type Rejection =  Response;
    
    async fn from_request_parts(
        parts: &mut Parts,
        state: &S
    ) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|rejection: QueryRejection| {
                let error = ErrorDetails {
                    title: String::from("Invalid query parameter"),
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    description: rejection.body_text(),
                    errors: None
                };

                (StatusCode::BAD_REQUEST, Json(error)).into_response()
            })?;

        Ok(Self(value))
    }
}