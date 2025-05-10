/* Api V1 routes module */

use axum::{Router, routing::post};

use crate::app_modules::AppState;
use crate::app_modules::api::v1::handlers::auth_handlers;

pub fn v1_routes() -> Router<AppState> {
    Router::new().route("/auth/signup", post(auth_handlers::local_signup))
}
