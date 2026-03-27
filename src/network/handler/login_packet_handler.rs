use std::collections::HashMap;
use std::io::{Cursor, Read};
use crate::network::session::Session;
use std::sync::{Arc, Weak};
use bedrockrs::proto::{ProtoCodecLE, V944};
use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::Query;
use tracing::{debug, info};
use uuid::Uuid;
use crate::network::handler::PacketReceivedMessage;
use crate::network::session::state::SessionState;
use crate::server::Server;

pub fn handle_login(
    mut events: MessageReader<PacketReceivedMessage>,
    mut sessions: Query<&mut Session>
) {
    for ev in events.read() {
        if let Ok(mut session) = sessions.get_mut(ev.entity) {
            if session.state != SessionState::Login { continue; }

            let V944::LoginPacket(packet) = &ev.packet else { continue; };

            debug!("Received LoginPacket: {:?}", packet);

            let mut req_bytes = Cursor::new(packet.connection_request.as_slice());
            decode_chain_data(&mut req_bytes);
        }
    }
}

pub struct ChainData {
    issue_time: i64,
    username: String,
    client_uuid: Uuid,
    title_id: String,
}

fn decode_chain_data(stream: &mut Cursor<&[u8]>) -> Option<ChainData> {
    let length = <i32 as ProtoCodecLE>::deserialize(stream).ok()?;

    let mut chain_buffer = Vec::<u8>::with_capacity(length as usize);
    stream.take(length as u64).read_to_end(&mut chain_buffer).ok()?;

    let chain_json = String::from_utf8(chain_buffer).ok()?;

    info!("Login json: {}", chain_json);

    let map = serde_json::from_str::<HashMap<String, Vec<String>>>(&chain_json).ok()?;

    if let Some(chains) = map.get("chain") {
        for chain in chains {

        }

        None
    } else { None }
}
