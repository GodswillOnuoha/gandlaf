use crate::app_modules::health;
use axum::{Router, routing::get};

pub fn build_app() -> Router {
    Router::new().route("/health", get(health::health_check))
}
