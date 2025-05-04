use gandalf::domain::models::User;

use mockall::{automock, predicate::*};

use async_trait::async_trait;
use gandalf::adapters::repositories::Result as RepositoryResult;
use gandalf::adapters::repositories::UserRepository;

pub struct _UserRepository {}

#[automock]
#[async_trait]
impl UserRepository for _UserRepository {
    async fn email_exists(&self, _email: &str) -> RepositoryResult<bool> {
        Ok(false)
    }

    async fn save(&self, _user: &User) -> RepositoryResult<()> {
        Ok(())
    }
}
