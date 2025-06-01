/* V1 use schemas module */

use crate::domain::models::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// User registration with email and password
#[derive(Debug, Deserialize, Validate)]
pub struct AuthLocal {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub email: String,
    pub external_id: Option<String>,
    pub email_verified: bool,
    pub auth_provider: String,
    pub user_state: String,
    pub requires_mfa: bool,
    pub created_at: String,
    pub last_login_at: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            user_id: user.id,
            username: user.username,
            email: user.email,
            external_id: user.external_id,
            email_verified: user.email_verified,
            auth_provider: user.auth_provider.to_string(),
            user_state: user.user_state.to_string(),
            requires_mfa: user.requires_mfa,
            created_at: user.created_at.to_rfc3339(),
            last_login_at: user.last_login_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use serde_json::json;

    #[test]
    fn test_user_response_schema() {
        let user_id = Uuid::parse_str("c21ff270-2b35-48fb-adcf-2b1756003c98").unwrap();
        let created_at: DateTime<Utc> = "2025-05-04T09:57:13.479118+00:00".parse().unwrap();

        let user = User {
            id: user_id,
            email: "test@mail.com".to_string(),
            created_at,
            ..User::default()
        };

        let user_response = UserResponse::from(user);

        let expected = json!({
            "userId": "c21ff270-2b35-48fb-adcf-2b1756003c98",
            "username": null,
            "email": "test@mail.com",
            "externalId": null,
            "emailVerified": false,
            "authProvider": "local",
            "userState": "registered",
            "requiresMfa": false,
            "createdAt": "2025-05-04T09:57:13.479118+00:00",
            "lastLoginAt": null
        });

        let actual = serde_json::to_value(user_response).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_auth_local_schema() {
        let auth_local = AuthLocal {
            email: "test@mail.com".to_string(),
            password: "123456789".to_string(),
        };

        let result = auth_local.validate();

        assert!(result.is_ok());
    }
}
