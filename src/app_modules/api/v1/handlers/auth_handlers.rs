/* V1 user handler module */

use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Json, State},
    http::HeaderMap,
    response::IntoResponse,
};

use tracing::{debug, error};

use crate::adapters::dtos::SignupDto;
use crate::app_modules::api::ResponseResult;
use crate::app_modules::api::v1::schemas::{AuthLocal, AuthResponse, UserResponse};
use crate::app_modules::{AppState, auth::AuthMethod};
use crate::utils::user_agent::get_device_info;

use crate::app_modules::api::AppError;

pub async fn local_login(
    state: State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(payload): Json<AuthLocal>,
) -> ResponseResult<impl IntoResponse> {
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

    // Extract the user agent and device information
    let ip = addr.ip();
    let device_info = get_device_info(headers);

    let (access_token, refresh_token) = state
        .auth_service
        .make_session(auth_user, ip, device_info)
        .await?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
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
