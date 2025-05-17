// Email/Password Registration Strategy

use std::sync::Arc;
use tracing::error;

use crate::adapters::dtos::{AuthUserDto, SignupDto};
use crate::app_modules::auth::strategies::AuthStrategy;
use crate::domain::models::AuthProvider;
use crate::domain::models::User;
use crate::domain::services::EmailService;
use crate::domain::services::UserService;
use crate::utils::PasswordUtil;

use super::super::{Error, Result};

pub struct EmailPasswordAuthStrategy {
    user_service: Arc<UserService>,
    email_service: Arc<EmailService>,
    password_util: Arc<PasswordUtil>,
}

impl EmailPasswordAuthStrategy {
    pub fn new(
        user_service: Arc<UserService>,
        email_service: Arc<EmailService>,
        password_util: Arc<PasswordUtil>,
    ) -> Self {
        Self {
            user_service,
            email_service,
            password_util,
        }
    }
}

#[async_trait::async_trait]
impl AuthStrategy for EmailPasswordAuthStrategy {
    async fn authenticate(&self, dto: &SignupDto) -> Result<AuthUserDto> {
        let SignupDto::EmailPassord { email, password } = dto;

        // Check if user exists
        let auth_user = self
            .user_service
            .find_auth_user(email)
            .await?
            .ok_or(Error::InvalidCredentials)?;

        // Verify password
        let verified = self
            .password_util
            .verify_password(password, &auth_user.password_hash)
            .map_err(|e| {
                error!("Password verification failed: {}", e);
                Error::InvalidCredentials
            })?;

        if !verified {
            return Err(Error::InvalidCredentials);
        }
        Ok(auth_user)
    }

    async fn signup(&self, dto: &SignupDto) -> Result<User> {
        let SignupDto::EmailPassord { email, password } = dto;

        // Validate email format
        self.email_service.validate_email(email);

        // Check if user already exists
        if self
            .user_service
            .user_exists(email)
            .await
            .map_err(|_| Error::InternalError)?
        {
            return Err(Error::UserAlreadyExists);
        }

        // Hash password
        let mut user = User::new(email.to_string());
        let password_result = self.password_util.hash_password(password);
        match password_result {
            Ok(hash) => user.password_hash = Some(hash),
            Err(e) => {
                error!("{}", e);
                user.password_hash = None
            }
        }

        user.auth_provider = AuthProvider::Local;

        self.user_service.save_user(&user).await?;

        // Generate verification token
        let verification_token = self
            .user_service
            .generate_email_verification_token(&user.id)
            .await?;

        // Send verification email asynchronously
        let email_clone = user.email.clone();
        tokio::spawn(async move {
            let email_service = EmailService::new();
            email_service
                .send_verification_email(email_clone, verification_token)
                .await
        });

        Ok(user)
    }
}
