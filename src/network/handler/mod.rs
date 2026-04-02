use crate::network::handler::encryption::handle_encryption;
use crate::network::handler::login::handle_login;
use crate::network::handler::resource_pack::handle_resource_pack;
use crate::network::handler::start_session::handle_start_session;
use bedrockrs::proto::V944;
use bevy_app::{App, FixedUpdate, Plugin};
use bevy_ecs::prelude::{Entity, Message};

pub mod encryption;
pub mod login;
pub mod resource_pack;
pub mod start_session;

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
                handle_encryption,
                handle_resource_pack,
            ),
        );
    }
}
