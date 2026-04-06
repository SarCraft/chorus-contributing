use crate::block::block_definition::BlockDefinition;
use crate::block::block_id;

const DIRT: BlockDefinition = BlockDefinition {
    identifier: block_id::DIRT,
    states: &[],
    components: &[],
    permutations: &[],
};
