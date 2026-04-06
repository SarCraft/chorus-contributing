use crate::block::block_definition::BlockDefinition;
use crate::block::block_id;

const BEDROCK: BlockDefinition = BlockDefinition {
    identifier: block_id::BEDROCK,
    states: &[],
    components: &[],
    permutations: &[],
};
