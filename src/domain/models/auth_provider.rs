/*
This module holds the model for supported auth providers
*/

use serde::{Deserialize, Serialize};
use std::fmt;

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
