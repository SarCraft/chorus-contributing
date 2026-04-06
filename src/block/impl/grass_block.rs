use crate::block::block_definition::BlockDefinition;
use crate::block::block_id;
use crate::block::component::mineable_component::MineableComponent;

pub const GRASS_BLOCK: BlockDefinition = BlockDefinition {
    identifier: block_id::GRASS_BLOCK,
    states: &[],
    components: &[&MineableComponent::hardness(0.6)],
    permutations: &[],
};
