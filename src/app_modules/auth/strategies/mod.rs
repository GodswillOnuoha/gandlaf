/* Authentication strategies */

mod email_password_strategy;
mod strategy;

pub use email_password_strategy::EmailPasswordAuthStrategy;
pub use strategy::AuthStrategy;
