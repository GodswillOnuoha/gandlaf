/*
Module for repositories
This module contains the repository interfaces and implementations for the application.
*/

mod errors;
pub mod user_repo;

pub use errors::{Error, Result};
pub use user_repo::{PgUserRepository, UserRepository};
