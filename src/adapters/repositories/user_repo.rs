/*
Module for user repository implementation
This module contains the UserRepository trait definition and its implementation for PostgreSQL.
*/

use async_trait::async_trait;
use std::sync::Arc;
use tokio_postgres::types::ToSql;

use super::Result;
use crate::adapters::dtos::AuthUserDto;
use crate::config::database::PgPool;
use crate::domain::models::User;

use tracing::debug;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<()>;
    async fn email_exists(&self, email: &str) -> Result<bool>;
}

// Postgres User Repository
pub struct PgUserRepository {
    pool: Arc<PgPool>,
}

impl PgUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn find_auth_user_query() -> &'static str {
        r#"
            SELECT
                id,
                external_id,
                email,
                password_hash,
                access_range
            FROM auth.users
            WHERE email = $1
        "#
    }

    fn email_exists_query() -> &'static str {
        r#"
            SELECT
                email
            FROM auth.users
            WHERE email = $1
        "#
    }

    fn save_user_query() -> &'static str {
        r#"
            INSERT INTO auth.users (
                id,
                external_id,
                username,
                email,
                password_hash,
                password_updated_at,
                password_reset_required,
                failed_login_attempts,
                last_failed_attempt,
                account_locked_until,
                email_verified,
                email_verification_token,
                email_verification_sent_at,
                created_at,
                updated_at,
                requires_mfa,
                auth_provider,
                user_state,
                access_range,
                deletion_scheduled_at
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20
            )
        "#
    }

    pub async fn find_auth_user(&self, id: &str) -> Result<Option<AuthUserDto>> {
        let conn = self.pool.get().await?;
        let params: &[&(dyn ToSql + Sync)] = &[&id];
        let result = conn.query_opt(Self::find_auth_user_query(), params).await?;

        match result {
            Some(row) => {
                debug!("row: {:?}", row);

                Ok(Some(AuthUserDto {
                    id: row.get("id"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    access_range: row.get("access_range"),
                }))
            }
            None => Ok(None),
        }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn email_exists(&self, email: &str) -> Result<bool> {
        let conn = self.pool.get().await?;
        let params: &[&(dyn ToSql + Sync)] = &[&email];
        let row = conn.query_opt(Self::email_exists_query(), params).await?;

        match row {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn save(&self, user: &User) -> Result<()> {
        let conn = self.pool.get().await?;

        let params: &[&(dyn ToSql + Sync)] = &[
            &user.id,
            &user.external_id,
            &user.username,
            &user.email,
            &user.password_hash,
            &user.password_updated_at,
            &user.password_reset_required,
            &user.failed_login_attempts,
            &user.last_failed_attempt,
            &user.account_locked_until,
            &user.email_verified,
            &user.email_verification_token,
            &user.email_verification_sent_at,
            &user.created_at,
            &user.updated_at,
            &user.requires_mfa,
            &user.auth_provider.to_string(),
            &user.user_state.to_string(),
            &user.access_range.to_string(),
            &user.deletion_scheduled_at,
        ];

        conn.execute(Self::save_user_query(), params).await?;
        Ok(())
    }
}
