use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ErrorDetails {
    pub title: String,
    pub status_code: u16,
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, Vec<JsonFieldError>>>,
}

#[derive(Debug, Serialize)]
pub struct JsonFieldError {
    pub message: Option<String>,
    pub params: HashMap<String, String>
}