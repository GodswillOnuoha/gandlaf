use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use thiserror::Error;

/// Password-related errors
#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password hashing failed: {0}")]
    Hashing(#[from] argon2::password_hash::Error),

    #[error("Invalid password hash")]
    InvalidHash,

    #[error("Password verification failed")]
    VerificationFailed,
}

/// Provides password hashing and verification using Argon2id.
pub struct PasswordUtil {
    hasher: Argon2<'static>,
}

impl PasswordUtil {
    /// Initializes a password utility with OWASP-recommended Argon2id parameters.
    pub fn new() -> Self {
        let memory_cost = 19_456; // 19 MB (OWASP recommended)
        let time_cost = 2;
        let parallelism = 2; // Use 2 threads for hashing
        let params = Params::new(memory_cost, time_cost, parallelism, None)
            .expect("Failed to initialize Argon2 parameters");

        let hasher = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        Self { hasher }
    }

    /// Hashes the given password and returns the encoded hash string.
    pub fn hash_password(&self, password: &str) -> Result<String, PasswordError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self.hasher.hash_password(password.as_bytes(), &salt)?;
        Ok(hash.to_string())
    }

    /// Verifies a password against a stored Argon2 hash string.
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, PasswordError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| PasswordError::InvalidHash)?;

        Ok(self
            .hasher
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

impl Default for PasswordUtil {
    fn default() -> Self {
        Self::new()
    }
}

/// Example usage and tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_success() {
        let util = PasswordUtil::new();
        let password = "secure_password";

        let hash = util.hash_password(password).expect("Should hash password");
        assert!(PasswordHash::new(&hash).is_ok());

        let result = util.verify_password(password, &hash);
        assert!(result.unwrap(), "Password should verify");
    }

    #[test]
    fn test_verify_password_failure() {
        let util = PasswordUtil::new();
        let password = "secure_password";
        let wrong_password = "wrong_password";

        let hash = util.hash_password(password).expect("Should hash password");
        let result = util.verify_password(wrong_password, &hash);

        assert!(!result.unwrap(), "Password verification should fail");
    }

    #[test]
    fn test_invalid_hash_format() {
        let util = PasswordUtil::new();
        let bad_hash = "not-a-real-hash";

        let result = util.verify_password("anything", bad_hash);
        assert!(matches!(result, Err(PasswordError::InvalidHash)));
    }
}
