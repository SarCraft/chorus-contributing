use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Player {
    unique_id: i64,
    runtime_id: u64,
}

impl Player {
    pub fn new(runtime_id: u64) -> Self {
        Self {
            unique_id: rand::random(),
            runtime_id,
        }
    }

    pub fn unique_id(&self) -> i64 {
        self.unique_id
    }

    pub fn runtime_id(&self) -> u64 {
        self.runtime_id
    }
}
