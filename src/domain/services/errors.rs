/* Services error module */

use crate::app_modules::api::AppError;
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

/// User service error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User not found")]
    UserNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Repository error: {0}")]
    RepositoryError(#[from] crate::adapters::repositories::Error),
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        match error {
            Error::UserNotFound => AppError::NotFound("User not found".to_string()),
            Error::UserAlreadyExists => AppError::BadRequest("User already exists".to_string()),
            Error::RepositoryError(err) => {
                error!("Repository error: {}", err);
                AppError::Internal("Internal server error".to_string())
            }
        }
    }
}
