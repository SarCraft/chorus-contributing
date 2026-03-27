use bedrockrs::proto::V944;
use bevy_app::{App, FixedUpdate, Plugin};
use bevy_ecs::prelude::{Entity, Message};
use crate::network::handler::login_packet_handler::handle_login;
use crate::network::handler::start_session_handler::handle_start_session;

pub mod login_packet_handler;
pub mod start_session_handler;

#[derive(Message)]
pub struct PacketReceivedMessage {
    pub entity: Entity,
    pub packet: V944,
}

pub struct PacketHandlers;

impl Plugin for PacketHandlers {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate, 
            (
                handle_start_session,
                handle_login,
            )
        );
    }
}