/* Api errors module */

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type ResponseResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Unauthorized(String),
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}
