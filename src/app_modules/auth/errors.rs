/* Auth errors module */

use crate::app_modules::api::AppError;
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

/// User service error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid Credentials")]
    InvalidCredentials,

    #[error("Invalid email")]
    InvalidEmail,

    #[error("User not found")]
    UserNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Internal server error")]
    InternalError,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Missing token")]
    MissingToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("User service error: {0}")]
    UserServiceError(#[from] crate::domain::services::errors::Error),
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        match error {
            Error::InvalidToken => AppError::BadRequest("Invalid token".to_string()),
            Error::MissingToken => AppError::BadRequest("Missing token".to_string()),
            Error::TokenExpired => AppError::BadRequest("Token expired".to_string()),
            Error::InvalidCredentials => {
                AppError::BadRequest("Wrong username or password".to_string())
            }
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
