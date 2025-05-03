/*
Module holds error for the repository layer
*/

use bb8::RunError;

pub type Result<T> = std::result::Result<T, Error>;

// A generic error type for all repositories
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database connection error: {0}")]
    ConnectionError(String),

    #[error("Database error: {0}")]
    DbError(#[from] tokio_postgres::Error),

    #[error("Pool error: {0}")]
    PoolError(#[from] RunError<tokio_postgres::Error>),
}
