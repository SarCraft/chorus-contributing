use crate::network::login::auth::auth_oidc::AuthOIDC;
use crate::network::login::auth::auth_payload::AuthPayload;
use crate::network::login::auth::auth_type::AuthType;
use serde::{Deserialize, Deserializer};

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
            let chain: Vec<String> =
                serde_json::from_str(&cert).map_err(serde::de::Error::custom)?;

            AuthPayload::Chain(chain)
        };

        Ok(Self {
            auth_payload,
            auth_type: identity.auth_type,
        })
    }
}

#[derive(Deserialize)]
pub struct AuthDataClaims {
    pub mid: String,
    pub xid: String,
    pub xname: String,
    pub cpk: String,
}

impl AuthData {
    pub fn validate(&self) -> Option<(bool, AuthDataClaims)> {
        match &self.auth_payload {
            AuthPayload::Chain(_) => {
                // TODO
                None
            }
            AuthPayload::Token(token) => Self::validate_token(token, &self.auth_type, None),
        }
    }

    fn validate_token(
        token: &String,
        auth_type: &AuthType,
        oidc: Option<AuthOIDC>,
    ) -> Option<(bool, AuthDataClaims)> {
        if let Some(oidc) = oidc {
            match auth_type {
                AuthType::Online | AuthType::Guest => {
                    return Self::validate_online_token(token, &oidc);
                }
                _ => {}
            }
        }
        Self::validate_offline_token(token)
    }

    fn validate_online_token(token: &String, oidc: &AuthOIDC) -> Option<(bool, AuthDataClaims)> {
        let header = jsonwebtoken::decode_header(token).ok()?;

        let jwk = oidc.jwks.find(&header.kid?)?;
        let key = jsonwebtoken::DecodingKey::from_jwk(jwk).ok()?;

        let mut validation = jsonwebtoken::Validation::new(header.alg);
        validation.set_audience(&[&oidc.audience]);
        validation.set_issuer(&[&oidc.issuer]);

        let data = jsonwebtoken::decode::<AuthDataClaims>(token, &key, &validation).ok()?;

        Some((true, data.claims))
    }

    fn validate_offline_token(token: &String) -> Option<(bool, AuthDataClaims)> {
        let data = jsonwebtoken::dangerous::insecure_decode::<AuthDataClaims>(token).ok()?;

        Some((false, data.claims))
    }
}
