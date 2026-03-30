use crate::network::login::auth::auth_type::AuthType;
use serde::{Deserialize, Deserializer};
use crate::network::login::auth::auth_payload::AuthPayload;

#[derive(Deserialize, Clone, Debug)]
struct RawAuthData {
    #[serde(rename = "AuthenticationType")]
    pub auth_type: AuthType,
    #[serde(rename = "Certificate")]
    pub certificate: Option<String>,
    #[serde(rename = "Token")]
    pub token: String,
}

#[derive(Clone, Debug)]
pub struct AuthData {
    pub auth_type: AuthType,
    pub auth_payload: AuthPayload,
}

impl<'de> Deserialize<'de> for AuthData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let identity = RawAuthData::deserialize(deserializer)?;
        let auth_payload = if !identity.token.is_empty() {
            AuthPayload::Token(identity.token)
        } else {
            let cert = identity.certificate.unwrap_or_default();
            let chain: Vec<String> = serde_json::from_str(&cert)
                .map_err(serde::de::Error::custom)?;

            AuthPayload::Chain(chain)
        };
        
        Ok(Self {
            auth_payload,
            auth_type: identity.auth_type,
        })
    }
}

pub struct AuthDataValidationResult {
    pub signed: bool,
    pub cpk: String,
}

impl AuthData {
    pub fn validate(&self) -> Option<AuthDataValidationResult> {
        match &self.auth_payload {
            AuthPayload::Chain(_) => {
                // TODO
                None
            },
            AuthPayload::Token(token) => {
                Self::validate_token(token, &self.auth_type)
            }
        }
    }
    
    fn validate_token(token: &String, auth_type: &AuthType) -> Option<AuthDataValidationResult> {
        match auth_type {
            AuthType::Offline => {
                
            }
            _ => {
                
            }
        }
        None
    }
}