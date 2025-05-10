/* Auth errors module */

use crate::app_modules::api::AppError;
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

/// User service error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid email")]
    InvalidEmail,

    #[error("User not found")]
    UserNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Internal server error")]
    InternalError,

    #[error("User service error: {0}")]
    UserServiceError(#[from] crate::domain::services::errors::Error),
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        match error {
            Error::InvalidEmail => AppError::BadRequest("Invalid email".to_string()),
            Error::UserNotFound => AppError::NotFound("User not found".to_string()),
            Error::UserAlreadyExists => AppError::BadRequest("User already exists".to_string()),
            Error::InternalError => AppError::Internal("Internal server error".to_string()),
            Error::UserServiceError(err) => {
                error!("{}", err);
                AppError::Internal("Internal server error".to_string())
            }
        }
    }
}
