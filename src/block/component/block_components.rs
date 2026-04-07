use crate::block::component::block_component::{AsAny, BlockComponent};
use atomicow::CowArc;
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BlockComponents {
    map: HashMap<TypeId, CowArc<'static, dyn BlockComponent>>,
}

impl Default for BlockComponents {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockComponents {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn insert(&mut self, component: CowArc<'static, dyn BlockComponent>) {
        self.map.insert(component.as_any().type_id(), component);
    }

    pub fn get<T: BlockComponent>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>()).and_then(|c| c.as_any().downcast_ref::<T>())
    }

    pub fn contains<T: BlockComponent>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }
}
