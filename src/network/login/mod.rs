use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use p384::ecdsa::VerifyingKey;
use p384::pkcs8::DecodePublicKey;

pub mod auth;

pub fn parse_cpk(cpk: &str) -> Option<VerifyingKey> {
    let der = BASE64_STANDARD.decode(cpk).ok()?;
    VerifyingKey::from_public_key_der(&der).ok()
}