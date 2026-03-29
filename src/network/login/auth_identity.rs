use serde::{Deserialize, Serialize};
use crate::network::login::auth_type::AuthType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthIdentity {
    #[serde(rename = "AuthenticationType")]
    auth_type: AuthType,
    #[serde(rename = "Certificate")]
    certificate: Option<String>,
    #[serde(rename = "Token")]
    token: String,
}