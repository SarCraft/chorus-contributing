use crate::block::block_definition::BlockDefinition;
use crate::block::block_id;

pub const BEDROCK: BlockDefinition = BlockDefinition {
    identifier: block_id::BEDROCK,
    states: &[],
    components: &[],
    permutations: &[],
};
