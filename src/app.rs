/* Moudle buolds application and state */

use std::sync::Arc;

use axum::{Router, routing::get};

use crate::app_modules::AppState;
use crate::app_modules::api::v1::routes::v1_routes;
use crate::app_modules::health;
use crate::config::database::PgPool;

pub fn build_app(db: Arc<PgPool>) -> Router {
    let app_state = AppState { db };

    Router::new()
        .route("/health", get(health::health_check))
        .with_state(app_state.clone())
        .nest("/api/v1", v1_routes().with_state(app_state))
}
