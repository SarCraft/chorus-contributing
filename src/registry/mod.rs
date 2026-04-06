use crate::registry::block_definition_registry::BlockRegistry;
use bevy_app::{App, Plugin, Startup};

pub mod block_definition_registry;

pub struct Registry;

impl Plugin for Registry {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, BlockRegistry::init);
    }
}
