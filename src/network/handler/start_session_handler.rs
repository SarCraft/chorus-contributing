use bedrockrs::proto::{ProtoVersion, V944};
use bedrockrs::proto::compression::Compression;
use bedrockrs::proto::v662::enums::{PacketCompressionAlgorithm, PlayStatus};
use bedrockrs::proto::v662::packets::NetworkSettingsPacket;
use log::debug;
use crate::network::handler::packet_handler::PacketHandler;
use crate::network::session::state::SessionState;
use crate::network::session::Session;

pub async fn handle(session: &mut Session, packet: V944) {
    let V944::RequestNetworkSettingsPacket(packet) = packet else { return; };

    debug!("Received RequestNetworkSettingsPacket: {:?}", packet);
    
    let protocol = packet.client_network_version as u32;

    if protocol != V944::PROTOCOL_VERSION {
        debug!("Disconnecting due to invalid protocol version: {}", protocol);
        
        session.send_play_status(
            if protocol < V944::PROTOCOL_VERSION {
                PlayStatus::LoginFailedClientOld
            } else {
                PlayStatus::LoginFailedServerOld
            },
            true
        ).await;

        session.close(
            if protocol < V944::PROTOCOL_VERSION {
                Some("disconnectionScreen.outdatedClient")
            } else {
                Some("disconnectionScreen.outdatedServer")
            }
        ).await;
    }

    debug!("Sending NetworkSettingsPacket");
    
    // TODO: IP Bans
    let mut conn = session.get_mut_connection_shard();
    conn.write(
        V944::NetworkSettingsPacket(
            NetworkSettingsPacket {
                compression_threshold: 1,
                compression_algorithm: PacketCompressionAlgorithm::None,
                client_throttle_enabled: false,
                client_throttle_threshold: 0,
                client_throttle_scalar: 0.0,
            }
        )
    ).await.unwrap();
    
    conn.get_mut_connection().await.compression = Some(Compression::None);

    debug!("Setting PacketHandler to LoginPacket");
    session.packet_handler = PacketHandler::LoginPacket;
    session.get_mut_state().handle(&SessionState::Login).await;
}