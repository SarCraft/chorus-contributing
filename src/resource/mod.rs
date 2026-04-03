use crate::resource::resource_pack::ResourcePack;
use bevy_ecs::prelude::*;

pub mod resource_pack;

#[derive(Resource)]
pub struct ResourcePacks {
    pub packs: Vec<ResourcePack>,
}
