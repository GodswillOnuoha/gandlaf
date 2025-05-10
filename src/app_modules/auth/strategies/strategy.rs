use crate::adapters::dtos::SignupDto;
use crate::domain::models::User;

use super::super::Result;

// Authentication Strategy Trait
#[async_trait::async_trait]
pub trait AuthStrategy {
    async fn signup(&self, dto: &SignupDto) -> Result<User>;
}
