use aes::Aes256;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use ctr::{Ctr128BE, cipher::KeyIvInit};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use p384::pkcs8::{EncodePrivateKey, EncodePublicKey};
use p384::{PublicKey, SecretKey};
use serde_json::json;
use sha2::{Digest, Sha256};

pub fn get_cipher(secret: &SecretKey, cpk: &PublicKey, token: &[u8; 16]) -> Ctr128BE<Aes256> {
    let shared = secret.diffie_hellman(&cpk);

    let shared_bytes = shared.raw_secret_bytes();

    let mut hasher = Sha256::new();
    hasher.update(&token);
    hasher.update(&shared_bytes);
    let hash = hasher.finalize();

    let mut iv = [0u8; 16];
    iv[..12].copy_from_slice(&hash[..12]);
    iv[15] = 2;

    Ctr128BE::<Aes256>::new(&hash, (&iv).into())
}

pub fn get_handshake_jwt(secret: &SecretKey, token: &[u8; 16]) -> Option<String> {
    let public = secret.public_key();
    let public_der = BASE64_STANDARD.encode(public.to_public_key_der().ok()?.as_bytes());

    let mut header = Header::new(Algorithm::ES384);
    header.typ = None;
    header.x5u = Some(public_der);

    let claims = json!({
        "salt": BASE64_STANDARD.encode(token)
    });

    let enc_key = EncodingKey::from_ec_der(secret.to_pkcs8_der().ok()?.as_bytes());

    encode(&header, &claims, &enc_key).ok()
}
