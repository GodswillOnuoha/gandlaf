/* User services module */

use crate::adapters::repositories::UserRepository;
use crate::domain::models::user::User;

use super::Result;
use super::errors::Error;

pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn register_user(&self, email: String, password: String) -> Result<User> {
        if self.repo.email_exists(&email).await? {
            return Err(Error::UserAlreadyExists);
        }

        let mut user = User::new(email);
        user.password_hash = Some(password);

        self.repo.save(&user).await?;
        Ok(user)
    }
}
