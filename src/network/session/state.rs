use bevy_ecs::prelude::{Entity, Message};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SessionState {
    Start,
    Login,
    ResourcePack,
    Encryption,
    PreSpawn,
    InGame,
    Death,
}

#[derive(Message, Clone, Debug)]
pub struct SessionStateChangedMessage {
    pub entity: Entity,
    pub from: SessionState,
    pub to: SessionState,
}
