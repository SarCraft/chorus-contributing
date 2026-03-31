use crate::config::ChorusConfig;
use crate::network::handler::PacketReceivedMessage;
use crate::network::login::auth::auth_identity::{AuthData, AuthDataClaims};
use crate::network::login::auth::auth_oidc::AuthOIDC;
use crate::network::session::Session;
use crate::network::session::state::SessionState;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use bedrockrs::proto::{ProtoCodecLE, V944};
use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::{Query, Res};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use p384::ecdsa::VerifyingKey;
use p384::pkcs8::DecodePublicKey;
use std::io::Read;
use tracing::*;

pub fn handle_login(
    config: Res<ChorusConfig>,
    oidc: Option<Res<AuthOIDC>>,
    mut events: MessageReader<PacketReceivedMessage>,
    mut sessions: Query<&mut Session>,
) {
    for ev in events.read() {
        let Ok(mut session) = sessions.get_mut(ev.entity) else {
            continue;
        };

        if session.state != SessionState::Login {
            continue;
        }

        let V944::LoginPacket(packet) = &ev.packet else {
            continue;
        };

        let Some(request) =
            decode_request(&mut packet.connection_request.as_slice(), oidc.as_deref())
        else {
            continue;
        };

        if (!request.online && config.online_mode) {
            session.close(Some("disconnectionScreen.notAuthenticated"));
            continue;
        }

        info!("Decoded RequestData: {:?}", request);
    }
}

#[derive(Clone, Debug)]
pub struct RequestData {
    online: bool,
    key: VerifyingKey,
    auth_data: AuthDataClaims,
    client_data: serde_json::Value,
}

fn decode_request<R: Read>(stream: &mut R, oidc: Option<&AuthOIDC>) -> Option<RequestData> {
    let auth_data_buf = {
        let len = <i32 as ProtoCodecLE>::deserialize(stream).ok()?;
        let mut buf = vec![0u8; len as usize];
        stream.read_exact(&mut buf).ok()?;
        buf
    };
    let auth_data = serde_json::from_slice::<AuthData>(&auth_data_buf).ok()?;

    let (online, claims) = auth_data.validate(oidc)?;

    let der = BASE64_STANDARD.decode(&claims.cpk).ok()?;
    let key = VerifyingKey::from_public_key_der(&der).ok()?;

    let client_data_buf = {
        let len = <i32 as ProtoCodecLE>::deserialize(stream).ok()?;
        let mut buf = vec![0u8; len as usize];
        stream.read_exact(&mut buf).ok()?;
        buf
    };

    // we use sec1 because DecodingKey uses `from_sec1_bytes` internally instead of `from_public_key_der`. misleading method name
    let dec_key = DecodingKey::from_ec_der(&key.to_sec1_bytes());

    let mut validator = Validation::new(Algorithm::ES384);
    validator.required_spec_claims.remove("exp");
    validator.validate_exp = false;

    let data = decode::<serde_json::Value>(&client_data_buf, &dec_key, &validator).ok()?;

    Some(RequestData {
        online,
        key,
        auth_data: claims,
        client_data: data.claims,
    })
}
