/* V1 user handler module */

use std::net::IpAddr;

use axum::{
    extract::{Json, State},
    response::IntoResponse,
};

use tracing::{debug, error};

use crate::adapters::dtos::SignupDto;
use crate::app_modules::api::ResponseResult;
use crate::app_modules::api::v1::schemas::{AuthLocal, AuthResponse, UserResponse};
use crate::app_modules::{AppState, auth::AuthMethod};

// use crate::app_modules::middleware::ClientIp;

use crate::app_modules::api::AppError;

pub async fn local_login(
    state: State<AppState>,
    Json(payload): Json<AuthLocal>,
) -> ResponseResult<impl IntoResponse> {
    // debug!("user login: {}, from ip: {}", payload.email, ip);

    let strategy = match state
        .auth_service
        .strategies
        .get(&AuthMethod::EmailPassword)
    {
        Some(strategy) => strategy,
        None => {
            error!("AuthMethod not found");
            return Err(AppError::BadRequest(
                "Authentication method not supported".to_string(),
            ));
        }
    };

    let auth_user = strategy
        .authenticate(&SignupDto::EmailPassord {
            email: payload.email,
            password: payload.password,
        })
        .await?;

    let ip: IpAddr = "192.168.1.1".parse().expect("Failed to parse IPv4");
    let token = state.auth_service.make_session(auth_user, ip).await?;

    Ok(Json(AuthResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        refresh_token: "refresh_token".to_string(),
    }))
}

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
