use serde::{Deserialize, Serialize};
use crate::network::login::auth_type::AuthType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthIdentity {
    #[serde(rename = "AuthenticationType")]
    pub auth_type: AuthType,
    #[serde(rename = "Certificate")]
    pub certificate: Option<String>,
    #[serde(rename = "Token")]
    pub token: String,
}