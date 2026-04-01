use crate::network::handler::PacketReceivedMessage;
use crate::network::session::state::SessionState;
use crate::network::session::Session;
use bedrockrs::proto::V944;
use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::Query;

pub fn handle_encryption(
    mut events: MessageReader<PacketReceivedMessage>,
    mut sessions: Query<&mut Session>,
) {
    for ev in events.read() {
        let Ok(mut session) = sessions.get_mut(ev.entity) else {
            continue;
        };

        if session.state != SessionState::Encryption {
            continue;
        };

        let V944::ClientToServerHandshakePacket(_) = &ev.packet else {
            continue;
        };

        session.state = SessionState::ResourcePack;
    }
}
