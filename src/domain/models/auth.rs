/*
This module holds the model for supported auth providers
*/

use std::fmt;
use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use std::collections::HashMap;

type ResourceAccess = HashMap<String, HashMap<String, Vec<String>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,          // User ID
    pub access_range: String, // Permissions/scope
    pub preferred_username: String,
    pub scope: String,
    pub sid: Uuid,                       // Session ID
    pub iss: String,                     // Issuer (auth server)
    pub aud: String,                     // Audience (client app)
    pub exp: i64,                        // Expiry time
    pub iat: i64,                        // Issued at
    pub jti: String,                     // Unique JWT ID (prevents replay attacks)
    pub nbf: i64,                        // Not before (optional)
    pub auth_time: i64,                  // Last authentication time
    pub resource_access: ResourceAccess, // Resource access permissions
}

impl JwtClaims {
    pub fn to_jwt(&self, secret: &str) -> String {
        let token = encode(
            &Header::new(Algorithm::HS256),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("Jwt Generation encoding should not fail");

        token
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub device_identifier: Option<String>,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub ip_address: IpAddr,
    pub user_agent: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub is_revoked: bool,
    pub revoked_reason: Option<String>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum AuthProvider {
    #[default]
    Local,
    Google,
    Microsoft,
    Apple,
    Facebook,
    Lti,
    Saml,
    Ldap,
    Custom,
}

impl std::str::FromStr for AuthProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(AuthProvider::Local),
            "google" => Ok(AuthProvider::Google),
            "microsoft" => Ok(AuthProvider::Microsoft),
            "apple" => Ok(AuthProvider::Apple),
            "facebook" => Ok(AuthProvider::Facebook),
            "lti" => Ok(AuthProvider::Lti),
            "saml" => Ok(AuthProvider::Saml),
            "ldap" => Ok(AuthProvider::Ldap),
            "custom" => Ok(AuthProvider::Custom),
            _ => Err(format!("Invalid auth provider: {}", s)),
        }
    }
}

impl fmt::Display for AuthProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Convert enum variant to string representation
        let provider_str = match self {
            AuthProvider::Local => "local",
            AuthProvider::Google => "google",
            AuthProvider::Microsoft => "microsoft",
            AuthProvider::Apple => "apple",
            AuthProvider::Facebook => "facebook",
            AuthProvider::Lti => "lti",
            AuthProvider::Saml => "saml",
            AuthProvider::Ldap => "ldap",
            AuthProvider::Custom => "custom",
        };
        write!(f, "{}", provider_str)
    }
}
