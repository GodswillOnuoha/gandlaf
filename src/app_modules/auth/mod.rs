/* Auth module */

mod auth_config;

pub mod errors;
pub mod strategies;

pub use auth_config::{AuthMethod, configure_auth_strategies};
pub use errors::{Error, Result};
pub use strategies::AuthStrategy;
