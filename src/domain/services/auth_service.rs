use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;

use chrono::{Duration, Utc};
use tracing::error;
use uuid::Uuid;

use crate::adapters::repositories::{PgSessionRepository, SessionRepository};
use crate::app_modules::auth::{AuthMethod, AuthStrategy};
use crate::config::database::PgPool;

use crate::domain::models::{JwtClaims, RefreshTokenClaims, TokenType};

use crate::adapters::dtos::{AuthUserDto, DeviceInfo};
use crate::app_modules::auth::errors::Error;
use crate::config::app_config::{AppConfig, get_config};
use crate::domain::models::Session;

type Result<T> = std::result::Result<T, Error>;

pub struct AuthService {
    pub strategies: HashMap<AuthMethod, Arc<dyn AuthStrategy + Send + Sync>>,
    session_repository: PgSessionRepository,
    config: &'static AppConfig,
}

impl AuthService {
    pub fn new(
        auth_strategies: HashMap<AuthMethod, Arc<dyn AuthStrategy + Send + Sync>>,
        db: Arc<PgPool>,
    ) -> Self {
        Self {
            strategies: auth_strategies,
            session_repository: PgSessionRepository::new(db.clone()),
            config: get_config(),
        }
    }

    pub async fn make_session(
        &self,
        user: AuthUserDto,
        ip: IpAddr,
        device_info: DeviceInfo,
    ) -> Result<(String, String)> {
        let now = Utc::now();

        // Generate expiration timestamps
        let access_exp = now
            .checked_add_signed(Duration::minutes(
                self.config.access_token_expiration.into(),
            ))
            .ok_or_else(|| {
                error!("Invalid access token expiration timestamp");
                Error::InternalError
            })?
            .timestamp();

        let refresh_exp = now
            .checked_add_signed(Duration::hours(self.config.refresh_token_expiration.into()))
            .ok_or_else(|| {
                error!("Invalid refresh token expiration timestamp");
                Error::InternalError
            })?
            .timestamp();

        let session_id = Uuid::new_v4();

        // Refresh token claims
        let refresh_claims = RefreshTokenClaims {
            sub: user.id.to_string(),
            session_id: session_id.to_string(),
            exp: refresh_exp,
            iat: now.timestamp(),
            token_type: TokenType::Refresh.to_string(),
        };

        // TODO: Replace this stubbed permissions structure with actual DB-driven logic
        let mut permissions = HashMap::new();
        permissions.insert(
            "test-tets".to_string(),
            vec![
                (
                    "channel/test".to_string(),
                    vec!["read".into(), "write".into()],
                ),
                (
                    "channel/test2".to_string(),
                    vec!["read".into(), "write".into()],
                ),
            ]
            .into_iter()
            .collect(),
        );

        let session_exp = now
            .checked_add_signed(Duration::minutes(
                self.config.refresh_token_expiration.into(),
            ))
            .ok_or_else(|| {
                error!("Invalid session expiration timestamp");
                Error::InternalError
            })?;

        let session = Session {
            id: session_id,
            user_id: user.id,
            refresh_token_hash: refresh_claims.to_jwt(&self.config.jwt_secret),
            device_identifier: None,
            device_name: Some(device_info.device_name),
            device_type: Some(device_info.device_type),
            ip_address: ip,
            user_agent: Some(device_info.os),
            expires_at: session_exp,
            created_at: now,
            last_active_at: now,
            is_revoked: false,
            revoked_reason: None,
            revoked_at: None,
        };

        // Persist session
        self.session_repository
            .create_session(&session)
            .await
            .map_err(|e| {
                error!("Failed to create session: {e}");
                Error::InternalError
            })?;

        // Access token claims
        let access_claims = JwtClaims {
            sub: user.id.to_string(),
            scope: user.access_range.clone(),
            sid: session.id,
            iss: self.config.app_host.clone(),
            aud: "app.teta".into(),
            exp: access_exp,
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            nbf: now.timestamp(),
            auth_time: now.timestamp(),
            resource_access: permissions,
            token_type: TokenType::Access.to_string(),
        };

        let access_token = access_claims.to_jwt(&self.config.jwt_secret);
        let refresh_token = refresh_claims.to_jwt(&self.config.jwt_secret);

        Ok((access_token, refresh_token))
    }
}
