/*
This module contains the default values for the application configuration.
These values are used when the optional configuration values are not found
in the environment.
 */

// Server defaults
pub const APP_NAME: &str = "gandalf";
pub const APP_ENV: &str = "development";
pub const APP_HOST: &str = "localhost";
pub const APP_PORT: u16 = 3000;

// Auth defaults
pub const JWT_EXPIRATION: u8 = 60;
pub const REFRESH_TOKEN_EXPIRATION: u8 = 30;
pub const ACCESS_TOKEN_EXPIRATION: u8 = 15;
pub const PASSWORD_RESET_EXPIRATION: u8 = 24;
pub const VERIFICATION_CODE_EXPIRATION: u8 = 24;
pub const MAX_FAILED_LOGIN_ATTEMPTS: u8 = 5;
pub const ACCOUNT_LOCKOUT_DURATION: u8 = 30;

// Db defaults
pub const MAX_DB_CONNECTIONS: u16 = 5;
