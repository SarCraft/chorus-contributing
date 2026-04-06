use crate::block::block_definition::BlockDefinition;
use crate::block::block_id;

pub const DIRT: BlockDefinition = BlockDefinition {
    identifier: block_id::DIRT,
    states: &[],
    components: &[],
    permutations: &[],
};
