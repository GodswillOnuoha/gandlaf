/* Api V1 routes module */

use axum::{Router, routing::post};

use crate::app_modules::AppState;
use crate::app_modules::api::v1::handlers::user_handler;

pub fn v1_routes() -> Router<AppState> {
    Router::new().route("/users/register", post(user_handler::create_user))
}
