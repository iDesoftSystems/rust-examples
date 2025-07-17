use std::borrow::Cow;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Serialize)]
pub struct Created<T> {
    pub id: T,
}

#[derive(Debug, Serialize)]
pub struct FailureResponse {
    pub message: String,
    pub errors: Vec<InvalidField>,
}

#[derive(Debug, Serialize)]
pub struct InvalidField {
    pub field: String,
    pub message: String,
    pub code: String,
}

impl InvalidField {
    pub fn new(field_name: &str, error: &str, code: &str) -> Self {
        Self {
            field: field_name.to_string(),
            code: code.to_string(),
            message: error.to_string(),
        }
    }
}

impl From<Vec<InvalidField>> for FailureResponse {
    fn from(value: Vec<InvalidField>) -> Self {
        Self {
            message: String::from("validation error"),
            errors: value,
        }
    }
}

impl From<String> for FailureResponse {
    fn from(message: String) -> Self {
        Self {
            message,
            errors: vec![],
        }
    }
}

impl<T> Created<T> {
    pub fn new(id: T) -> Self {
        Self { id }
    }
}

impl<T> IntoResponse for Created<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

pub struct InternalServerError;

impl IntoResponse for InternalServerError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(FailureResponse::from("internal server error".to_string())),
        )
            .into_response()
    }
}

pub struct BadRequest(pub String);

impl IntoResponse for BadRequest {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(FailureResponse::from(self.0))).into_response()
    }
}

pub struct Conflict(pub String);

impl IntoResponse for Conflict {
    fn into_response(self) -> Response {
        (StatusCode::CONFLICT, Json(FailureResponse::from(self.0))).into_response()
    }
}

pub struct BadRequestWithErrors(pub ValidationErrors);

impl IntoResponse for BadRequestWithErrors {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            Json(FailureResponse::from(extract_validation_errors(self.0))),
        )
            .into_response()
    }
}

fn extract_validation_errors(errs: ValidationErrors) -> Vec<InvalidField> {
    let mut val_errors: Vec<InvalidField> = errs
        .field_errors()
        .into_iter()
        .map(|error| {
            let default_message = Cow::from("Invalid information");
            let field_message = error.1[0].message.as_ref().unwrap_or(&default_message);

            InvalidField::new(error.0.as_ref(), field_message, error.1[0].code.as_ref())
        })
        .collect();

    val_errors.sort_by(|a, b| a.field.to_lowercase().cmp(&b.field.to_lowercase()));

    val_errors
}
