/* V1 user handler module */

use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use tracing::debug;

use crate::adapters::dtos::SignupDto;
use crate::app_modules::api::ResponseResult;
use crate::app_modules::api::v1::schemas::{AuthLocal, UserResponse};
use crate::app_modules::{AppState, auth::AuthMethod};

use crate::app_modules::api::AppError;

pub async fn local_signup(
    State(state): State<AppState>,
    Json(payload): Json<AuthLocal>,
) -> ResponseResult<impl IntoResponse> {
    debug!("user registration: {}", payload.email);

    let strategy = match state
        .auth_service
        .strategies
        .get(&AuthMethod::EmailPassword)
    {
        Some(strategy) => strategy,
        None => {
            return Err(AppError::BadRequest(
                "Authentication method not supported".to_string(),
            ));
        }
    };

    let user = strategy
        .signup(&SignupDto::EmailPassord {
            email: payload.email,
            password: payload.password,
        })
        .await?;

    Ok(Json(UserResponse::from(user)))
}
