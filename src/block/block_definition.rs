use crate::block::component::block_component::BlockComponent;
use crate::block::state::block_state::{BlockState, BlockStateDefinition};
use std::collections::HashMap;

pub struct BlockDefinition {
    pub identifier: &'static str,
    pub states: &'static [&'static BlockStateDefinition],
    pub components: &'static [&'static dyn BlockComponent],
    pub permutations: &'static [&'static BlockPermutationDefinition],
}

pub struct BlockPermutationDefinition {
    pub condition: fn(&HashMap<&'static str, BlockState>) -> bool,
    pub components: &'static [&'static dyn BlockComponent],
}
