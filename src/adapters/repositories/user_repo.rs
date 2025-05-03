/*
Module for user repository implementation
This module contains the UserRepository trait definition and its implementation for PostgreSQL.
*/

use async_trait::async_trait;
use std::sync::Arc;
use tokio_postgres::types::ToSql;

use super::Result;
use crate::config::database::PgPool;
use crate::domain::models::user::User;

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
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn email_exists(&self, email: &str) -> Result<bool> {
        let conn = self.pool.get().await?;

        let query: &str = r#"
            SELECT
                email
            FROM auth.users
            WHERE email = $1
        "#;

        let params: &[&(dyn ToSql + Sync)] = &[&email];

        let row = conn.query_opt(query, params).await?;

        match row {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn save(&self, user: &User) -> Result<()> {
        let conn = self.pool.get().await?;

        let query: &str = r#"
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
                last_login_at,
                requires_mfa,
                auth_provider,
                user_state,
                last_login_ip,
                last_user_agent,
                deletion_scheduled_at
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20,
                $21, $22
            )
        "#;

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
            &user.last_login_at,
            &user.requires_mfa,
            &user.auth_provider.to_string(), // convert enum to string or int
            &user.user_state.to_string(),    // same
            &user.last_login_ip,             // .map(|ip| ip.to_string()),
            &user.last_user_agent,
            &user.deletion_scheduled_at,
        ];

        conn.execute(query, params).await?;
        Ok(())
    }
}
