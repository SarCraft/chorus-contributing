use crate::network::handler::PacketReceivedMessage;
use crate::network::session::Session;
use crate::network::session::state::{SessionState, SessionStateChangedMessage};
use bedrockrs::network::compression::Compression;
use bedrockrs::proto::v662::enums::{PacketCompressionAlgorithm, PlayStatus};
use bedrockrs::proto::v662::packets::NetworkSettingsPacket;
use bedrockrs::proto::{ProtoVersion, V944};
use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::MessageWriter;
use bevy_ecs::system::Query;
use tracing::error;

pub fn handle_request(
    mut reader: MessageReader<PacketReceivedMessage>,
    mut writer: MessageWriter<SessionStateChangedMessage>,
    mut sessions: Query<&mut Session>,
) {
    for ev in reader.read() {
        if let Ok(mut session) = sessions.get_mut(ev.entity) {
            if session.get_state() != SessionState::Request {
                continue;
            }

            let V944::RequestNetworkSettingsPacket(packet) = &ev.packet else {
                continue;
            };

            let protocol = packet.client_network_version as u32;

            if protocol != V944::PROTOCOL_VERSION {
                session.send_play_status(
                    if protocol < V944::PROTOCOL_VERSION {
                        PlayStatus::LoginFailedClientOld
                    } else {
                        PlayStatus::LoginFailedServerOld
                    },
                    true,
                );

                session.close(if protocol < V944::PROTOCOL_VERSION {
                    Some("disconnectionScreen.outdatedClient")
                } else {
                    Some("disconnectionScreen.outdatedServer")
                });
            }

            // TODO: IP Bans
            session.send_immediate(V944::NetworkSettingsPacket(NetworkSettingsPacket {
                compression_threshold: 1,
                compression_algorithm: PacketCompressionAlgorithm::None,
                client_throttle_enabled: false,
                client_throttle_threshold: 0,
                client_throttle_scalar: 0.0,
            }));

            session.set_compression(Some(Compression::None));

            session.set_state(SessionState::Login, &mut writer);
        } else {
            error!("received PacketReceivedMessage from entity without a Session!")
        }
    }
}
