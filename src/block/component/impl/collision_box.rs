use crate::block::component::block_component::BlockComponent;
use vek::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub struct CollisionBox {
    origin: Vec3<f32>,
    size: Vec3<f32>,
    enabled: bool,
}

impl BlockComponent for CollisionBox {}

impl CollisionBox {
    pub const fn default() -> CollisionBox {
        Self {
            origin: Vec3::new(-8.0, 0.0, -8.0),
            size: Vec3::new(16.0, 16.0, 16.0),
            enabled: true,
        }
    }

    pub const fn new(origin: Vec3<f32>, size: Vec3<f32>) -> CollisionBox {
        Self {
            origin,
            size,
            enabled: true,
        }
    }

    pub const fn enabled(enabled: bool) -> CollisionBox {
        Self {
            origin: Vec3::new(-8.0, 0.0, -8.0),
            size: Vec3::new(16.0, 16.0, 16.0),
            enabled,
        }
    }
}
