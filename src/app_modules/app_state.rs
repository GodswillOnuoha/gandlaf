/* Application state module */

use std::sync::Arc;

use crate::config::database::PgPool;
use crate::domain::services::AuthService;
use crate::domain::services::EmailService;
use crate::domain::services::UserService;

use crate::app_modules::auth::configure_auth_strategies;
use crate::utils::PasswordUtil;

// Configuration struct to hold application state
#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    pub fn new(db_pool: Arc<PgPool>) -> AppState {
        let user_service = Arc::new(UserService::new(db_pool.clone()));

        let email_service = Arc::new(EmailService::new());

        let auth_strategies = configure_auth_strategies(
            Arc::clone(&user_service),
            Arc::clone(&email_service),
            Arc::new(PasswordUtil::new()),
        );

        let auth_service = Arc::new(AuthService::new(auth_strategies));

        AppState {
            user_service,
            auth_service,
        }
    }
}
