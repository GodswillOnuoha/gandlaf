/* User services module */

use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::repositories::{PgUserRepository, UserRepository};
use crate::config::database::PgPool;
use crate::domain::models::User;

use super::errors::Error;

type Result<T> = std::result::Result<T, Error>;

pub struct UserService {
    repo: PgUserRepository,
}

impl UserService {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self {
            repo: PgUserRepository::new(db_pool),
        }
    }

    pub async fn user_exists(&self, email: &str) -> Result<bool> {
        let exists = self.repo.email_exists(email).await?;
        Ok(exists)
    }

    pub async fn save_user(&self, user: &User) -> Result<()> {
        self.repo.save(user).await?;
        Ok(())
    }

    pub async fn generate_email_verification_token(&self, _user_id: &Uuid) -> Result<String> {
        // Todo: generate a token and save it to the database
        // let token = Uuid::new_v4().to_string();
        Ok("some very long verification token".to_string())
    }
}
