use crate::block::block_definition::BlockDefinition;
use crate::block::block_id;
use crate::block::component::r#impl::collision_box::CollisionBox;

const AIR: BlockDefinition = BlockDefinition {
    identifier: block_id::AIR,
    states: &[],
    components: &[&CollisionBox::enabled(false)],
    permutations: &[],
};
