/*
This module contains config settings for authentication.
*/

use super::defaults;
use std::env;
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub app_name: String,
    pub app_host: String,
    pub app_port: u16,
    pub app_env: String,
    pub jwt_secret: String,
    pub jwt_expiration: u8,               // minutes
    pub refresh_token_expiration: u8,     // hours
    pub access_token_expiration: u8,      // minutes
    pub password_reset_expiration: u8,    // minutes
    pub verification_code_expiration: u8, // minutes
    pub max_failed_login_attempts: u8,
    pub account_lockout_duration: u8, // minutes
}

impl AppConfig {
    fn from_env() -> Self {
        Self {
            // Server settings
            app_name: get_env_or_default("APP_NAME", defaults::APP_NAME.to_string()),
            app_host: get_env_or_default("APP_HOST", defaults::APP_HOST.to_string()),
            app_port: get_env_or_default("APP_PORT", defaults::APP_PORT),

            // Environment settings
            app_env: get_env_or_default("APP_ENV", defaults::APP_ENV.to_string()),

            // JWT settings
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expiration: get_env_or_default("JWT_EXPIRATION", defaults::JWT_EXPIRATION),

            // Authentication settings
            refresh_token_expiration: get_env_or_default(
                "REFRESH_TOKEN_EXPIRATION",
                defaults::REFRESH_TOKEN_EXPIRATION,
            ),
            access_token_expiration: get_env_or_default(
                "ACCESS_TOKEN_EXPIRATION",
                defaults::ACCESS_TOKEN_EXPIRATION,
            ),
            password_reset_expiration: get_env_or_default(
                "PASSWORD_RESET_EXPIRATION",
                defaults::PASSWORD_RESET_EXPIRATION,
            ),
            verification_code_expiration: get_env_or_default(
                "VERIFICATION_CODE_EXPIRATION",
                defaults::VERIFICATION_CODE_EXPIRATION,
            ),
            max_failed_login_attempts: get_env_or_default(
                "MAX_FAILED_LOGIN_ATTEMPTS",
                defaults::MAX_FAILED_LOGIN_ATTEMPTS,
            ),
            account_lockout_duration: get_env_or_default(
                "ACCOUNT_LOCKOUT_DURATION",
                defaults::ACCOUNT_LOCKOUT_DURATION,
            ),
        }
    }
}

fn get_env_or_default<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr + ToString,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    env::var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<T>()
        .unwrap_or_else(|_| panic!("{key} must be a valid {}", std::any::type_name::<T>()))
}

// Global singleton config
static CONFIG_INSTANCE: OnceLock<AppConfig> = OnceLock::new();

pub fn get_config() -> &'static AppConfig {
    CONFIG_INSTANCE.get_or_init(AppConfig::from_env)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    fn set_env(key: &str, value: &str) {
        unsafe {
            env::set_var(key, value);
        }
    }

    #[test]
    #[serial] // Ensures this runs in isolation due to OnceLock
    fn test_app_config_defaults() {
        // Set required environment variables
        set_env("JWT_SECRET", "supersecret");
        set_env("JWT_EXPIRATION", "15");

        // Trigger config load
        let config = AppConfig::from_env();

        // Validate values
        assert_eq!(config.app_name, "gandalf");
        assert_eq!(config.app_host, "localhost");
        assert_eq!(config.app_port, 3000);
        assert_eq!(config.app_env, "development");
        assert_eq!(config.jwt_secret, "supersecret");
        assert_eq!(config.jwt_expiration, 15);
        assert_eq!(config.refresh_token_expiration, 30);
        assert_eq!(config.access_token_expiration, 15);
        assert_eq!(config.password_reset_expiration, 24);
        assert_eq!(config.verification_code_expiration, 24);
        assert_eq!(config.max_failed_login_attempts, 5);
        assert_eq!(config.account_lockout_duration, 30);
    }
}
