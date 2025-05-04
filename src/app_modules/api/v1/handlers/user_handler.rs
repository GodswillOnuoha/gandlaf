/* V1 user handler module */

use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use tracing::debug;

use crate::adapters::repositories::PgUserRepository;
use crate::app_modules::AppState;
use crate::app_modules::api::ResponseResult;
use crate::app_modules::api::v1::schemas::{RegistrationRequestLocal, UserResponse};
use crate::domain::services::UserService;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<RegistrationRequestLocal>,
) -> ResponseResult<impl IntoResponse> {
    debug!("user registration: {}", payload.email);

    // Instantiate repo from shared db pool
    let user_repo = PgUserRepository::new(state.db.clone());

    // Inject repo into service
    let user_service = UserService::new(user_repo);

    // Call service to create user
    let user = user_service
        .register_user(payload.email, payload.password)
        .await?;

    Ok(Json(UserResponse::from(user)))
}
