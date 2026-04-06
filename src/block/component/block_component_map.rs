use crate::block::component::block_component::{AsAny, BlockComponent};
use std::any::TypeId;
use std::collections::HashMap;

pub struct BlockComponentMap {
    map: HashMap<TypeId, &'static dyn BlockComponent>,
}

impl BlockComponentMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert<T: BlockComponent>(&mut self, component: &'static T) {
        self.map.insert(TypeId::of::<T>(), component);
    }

    pub fn get<T: BlockComponent>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|c| c.as_any().downcast_ref::<T>())
    }

    pub fn contains<T: BlockComponent>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }
}
