use std::io::{Cursor, Read};
use crate::network::session::Session;
use bedrockrs::proto::{ProtoCodecLE, V944};
use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::Query;
use serde::{Deserialize, Serialize};
use tracing::{info};
use uuid::Uuid;
use crate::network::handler::PacketReceivedMessage;
use crate::network::login::auth::auth_identity::{AuthData};
use crate::network::login::auth::auth_type::AuthType;
use crate::network::session::state::SessionState;

pub fn handle_login(
    mut events: MessageReader<PacketReceivedMessage>,
    mut sessions: Query<&mut Session>
) {
    for ev in events.read() {
        if let Ok(mut session) = sessions.get_mut(ev.entity) {
            if session.state != SessionState::Login { continue; }

            let V944::LoginPacket(packet) = &ev.packet else { continue; };
            
            let mut req_bytes = Cursor::new(packet.connection_request.as_slice());
            decode_identity(&mut req_bytes);
        }
    }
}

pub struct ChainData {
    issue_time: i64,
    username: String,
    client_uuid: Uuid,
    title_id: String,
}

fn decode_identity(stream: &mut Cursor<&[u8]>) -> Option<ChainData> {
    let length = <i32 as ProtoCodecLE>::deserialize(stream).ok()?;

    let mut identity_buf = Vec::<u8>::with_capacity(length as usize);
    stream.take(length as u64).read_to_end(&mut identity_buf).ok()?;

    let auth_data_json = String::from_utf8(identity_buf).ok()?;
    let auth_data = serde_json::from_str::<AuthData>(&auth_data_json).ok()?;
    info!("Login AuthData: {:?}", auth_data);
    
    let validated = auth_data.validate();
    
    None
}