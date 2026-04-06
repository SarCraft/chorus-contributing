use crate::block::block_definition::BlockDefinition;
use crate::block::block_id;

pub const GRASS_BLOCK: BlockDefinition = BlockDefinition {
    identifier: block_id::GRASS_BLOCK,
    states: &[],
    components: &[],
    permutations: &[],
};
