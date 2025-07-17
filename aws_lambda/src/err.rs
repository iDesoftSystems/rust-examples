use axum::response::{IntoResponse, Response};
use lambda_http::tracing;
use sea_orm::sqlx::Error;
use sea_orm::{DbErr, RuntimeErr, sqlx::error::ErrorKind};
use thiserror::Error;
use validator::ValidationErrors;

use crate::responses::{BadRequest, BadRequestWithErrors, Conflict, InternalServerError};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    DbError(#[from] DbErr),

    #[error(transparent)]
    Validation(#[from] ValidationErrors),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::DbError(db_err) => db_error_handler(&db_err),
            ApiError::Validation(validation_errors) => {
                BadRequestWithErrors(validation_errors).into_response()
            }
        }
    }
}

fn db_error_handler(err: &DbErr) -> Response {
    tracing::error!(?err, "database error");
    match err {
        DbErr::Exec(RuntimeErr::SqlxError(Error::Database(database_err))) => {
            match database_err.kind() {
                ErrorKind::UniqueViolation => {
                    Conflict(database_err.message().to_string()).into_response()
                }
                ErrorKind::ForeignKeyViolation => {
                    BadRequest("Associated information not found or invalid.".into())
                        .into_response()
                }
                _ => InternalServerError.into_response(),
            }
        }
        _ => InternalServerError.into_response(),
    }
}
