use crate::block::state::block_state::BlockStateDefinition;
use crate::block::state::r#impl::enums::attachment::Attachment;
use crate::block::state::r#impl::enums::bamboo_leaf_size::BambooLeafSize;
use crate::block::state::r#impl::enums::bamboo_stalk_thickness::BambooStalkThickness;
use crate::block::state::r#impl::enums::big_dripleaf_tilt::BigDripleafTilt;
use crate::block::state::r#impl::enums::cauldron_liquid::CauldronLiquid;
use crate::block::state::r#impl::enums::chemistry_table_type::ChemistryTableType;
use crate::block::state::r#impl::enums::chisel_type::ChiselType;
use crate::block::state::r#impl::enums::color::Color;
use crate::block::state::r#impl::enums::coral_color::CoralColor;
use crate::block::state::r#impl::enums::cracked_state::CrackedState;
use crate::block::state::r#impl::enums::creaking_heart_state::CreakingHeartState;
use crate::block::state::r#impl::enums::damage::Damage;
use crate::block::state::r#impl::enums::dirt_type::DirtType;
use crate::block::state::r#impl::enums::double_plant_type::DoublePlantType;
use crate::block::state::r#impl::enums::dripstone_thickness::DripstoneThickness;
use crate::block::state::r#impl::enums::level_direction::LeverDirection;
use crate::block::state::r#impl::enums::minecraft_cardinal_direction::MinecraftCardinalDirection;
use crate::block::state::r#impl::enums::minecraft_vertical_half::MinecraftVerticalHalf;
use crate::block::state::r#impl::enums::monster_egg_stone_type::MonsterEggStoneType;
use crate::block::state::r#impl::enums::new_leaf_type::NewLeafType;
use crate::block::state::r#impl::enums::old_leaf_type::OldLeafType;
use crate::block::state::r#impl::enums::orientation::Orientation;
use crate::block::state::r#impl::enums::pale_moss_carpet_side::PaleMossCarpetSide;
use crate::block::state::r#impl::enums::portal_axis::PortalAxis;
use crate::block::state::r#impl::enums::prismarine_block_type::PrismarineBlockType;
use crate::block::state::r#impl::enums::sand_type::SandType;
use crate::block::state::r#impl::enums::sea_grass_type::SeaGrassType;
use crate::block::state::r#impl::enums::sponge_type::SpongeType;
use crate::block::state::r#impl::enums::stone_brick_type::StoneBrickType;
use crate::block::state::r#impl::enums::stone_slab_type::StoneSlabType;
use crate::block::state::r#impl::enums::stone_slab_type_2::StoneSlabType2;
use crate::block::state::r#impl::enums::stone_slab_type_3::StoneSlabType3;
use crate::block::state::r#impl::enums::stone_slab_type_4::StoneSlabType4;
use crate::block::state::r#impl::enums::structure_block_type::StructureBlockType;
use crate::block::state::r#impl::enums::structure_void_type::StructureVoidType;
use crate::block::state::r#impl::enums::tall_grass_type::TallGrassType;
use crate::block::state::r#impl::enums::torch_facing_direction::TorchFacingDirection;
use crate::block::state::r#impl::enums::turtle_egg_count::TurtleEggCount;
use crate::block::state::r#impl::enums::vault_state::VaultState;
use crate::block::state::r#impl::enums::wall_block_type::WallBlockType;
use crate::block::state::r#impl::enums::wall_connection_type::WallConnectionType;
use crate::block::state::r#impl::enums::wood_type::WoodType;
use crate::math::enums::axis::Axis;
use crate::math::enums::block_face::BlockFace;
use strum::VariantNames;

pub const ACTIVE: BlockStateDefinition = BlockStateDefinition::new_bool("active", false);
pub const AGE_16: BlockStateDefinition = BlockStateDefinition::new_int("age", 0, 15);
pub const AGE_6: BlockStateDefinition = BlockStateDefinition::new_int("age", 0, 5);
pub const AGE_4: BlockStateDefinition = BlockStateDefinition::new_int("age", 0, 3);
pub const AGE_3: BlockStateDefinition = BlockStateDefinition::new_int("age", 0, 2);
pub const AGE_BIT: BlockStateDefinition = BlockStateDefinition::new_bool("age_bit", false);
pub const ALLOW_UNDERWATER_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("allow_underwater_bit", false);
pub const ATTACHED_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("attached_bit", false);
pub const ATTACHMENT: BlockStateDefinition =
    BlockStateDefinition::new_enum("attachment", Attachment::VARIANTS);
pub const BAMBOO_LEAF_SIZE: BlockStateDefinition =
    BlockStateDefinition::new_enum("bamboo_leaf_size", BambooLeafSize::VARIANTS);
pub const BAMBOO_STALK_THICKNESS: BlockStateDefinition =
    BlockStateDefinition::new_enum("bamboo_stalk_thickness", BambooStalkThickness::VARIANTS);
pub const BIG_DRIPLEAF_HEAD: BlockStateDefinition =
    BlockStateDefinition::new_bool("big_dripleaf_head", false);
pub const BIG_DRIPLEAF_TILT: BlockStateDefinition =
    BlockStateDefinition::new_enum("big_dripleaf_tilt", BigDripleafTilt::VARIANTS);
pub const BITE_COUNTER: BlockStateDefinition = BlockStateDefinition::new_int("bite_counter", 0, 6);
pub const BLOCK_LIGHT_LEVEL: BlockStateDefinition =
    BlockStateDefinition::new_int("block_light_level", 0, 15);
pub const BLOOM: BlockStateDefinition = BlockStateDefinition::new_bool("bloom", false);
pub const BOOKS_STORED: BlockStateDefinition = BlockStateDefinition::new_int("books_stored", 0, 63);
pub const BREWING_STAND_SLOT_A_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("brewing_stand_slot_a_bit", false);
pub const BREWING_STAND_SLOT_B_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("brewing_stand_slot_b_bit", false);
pub const BREWING_STAND_SLOT_C_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("brewing_stand_slot_c_bit", false);
pub const BRUSHED_PROGRESS: BlockStateDefinition =
    BlockStateDefinition::new_int("brushed_progress", 0, 3);
pub const BUTTON_PRESSED_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("button_pressed_bit", false);
pub const CAN_SUMMON: BlockStateDefinition = BlockStateDefinition::new_bool("can_summon", false);
pub const CANDLES: BlockStateDefinition = BlockStateDefinition::new_int("candles", 0, 3);
pub const CAULDRON_LIQUID: BlockStateDefinition =
    BlockStateDefinition::new_enum("cauldron_liquid", CauldronLiquid::VARIANTS);
pub const CHEMISTRY_TABLE_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("chemistry_table_type", ChemistryTableType::VARIANTS);
pub const CHISEL_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("chisel_type", ChiselType::VARIANTS);
pub const CLUSTER_COUNT: BlockStateDefinition =
    BlockStateDefinition::new_int("cluster_count", 0, 3);
pub const COLOR: BlockStateDefinition = BlockStateDefinition::new_enum("color", Color::VARIANTS);
pub const COLOR_BIT: BlockStateDefinition = BlockStateDefinition::new_bool("color_bit", false);
pub const COMPOSTER_FILL_LEVEL: BlockStateDefinition =
    BlockStateDefinition::new_int("composter_fill_level", 0, 8);
pub const CONDITIONAL_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("conditional_bit", false);
pub const CORAL_DIRECTION: BlockStateDefinition =
    BlockStateDefinition::new_int("coral_direction", 0, 3);
pub const CORAL_FAN_DIRECTION: BlockStateDefinition =
    BlockStateDefinition::new_int("coral_fan_direction", 0, 1);
pub const CORAL_HANG_TYPE_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("coral_hang_type_bit", false);
pub const COVERED_BIT: BlockStateDefinition = BlockStateDefinition::new_bool("covered_bit", false);
pub const CRACKED_STATE: BlockStateDefinition =
    BlockStateDefinition::new_enum("cracked_state", CrackedState::VARIANTS);
pub const CRAFTING: BlockStateDefinition = BlockStateDefinition::new_bool("crafting", false);
pub const CREAKING_HEART_STATE: BlockStateDefinition =
    BlockStateDefinition::new_enum("creaking_heart_state", CreakingHeartState::VARIANTS);
pub const DAMAGE: BlockStateDefinition = BlockStateDefinition::new_enum("damage", Damage::VARIANTS);
pub const DEAD_BIT: BlockStateDefinition = BlockStateDefinition::new_bool("dead_bit", false);
pub const DEPRECATED: BlockStateDefinition = BlockStateDefinition::new_int("deprecated", 0, 3);
pub const DIRECTION: BlockStateDefinition = BlockStateDefinition::new_int("direction", 0, 3);
pub const DIRT_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("dirt_type", DirtType::VARIANTS);
pub const DISARMED_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("disarmed_bit", false);
pub const DOOR_HINGE_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("door_hinge_bit", false);
pub const DOUBLE_PLANT_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("double_plant_type", DoublePlantType::VARIANTS);
pub const DRAG_DOWN: BlockStateDefinition = BlockStateDefinition::new_bool("drag_down", false);
pub const DRIPSTONE_THICKNESS: BlockStateDefinition =
    BlockStateDefinition::new_enum("dripstone_thickness", DripstoneThickness::VARIANTS);
pub const END_PORTAL_EYE_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("end_portal_eye_bit", false);
pub const EXPLODE_BIT: BlockStateDefinition = BlockStateDefinition::new_bool("explode_bit", false);
pub const EXTINGUISHED: BlockStateDefinition =
    BlockStateDefinition::new_bool("extinguished", false);
pub const CORAL_COLOR: BlockStateDefinition =
    BlockStateDefinition::new_enum("coral_color", CoralColor::VARIANTS);
pub const FACING_DIRECTION: BlockStateDefinition =
    BlockStateDefinition::new_int("facing_direction", 0, 5);
pub const FILL_LEVEL: BlockStateDefinition = BlockStateDefinition::new_int("fill_level", 0, 6);
pub const GROUND_SIGN_DIRECTION: BlockStateDefinition =
    BlockStateDefinition::new_int("ground_sign_direction", 0, 15);
pub const GROWING_PLANT_AGE: BlockStateDefinition =
    BlockStateDefinition::new_int("growing_plant_age", 0, 25);
pub const GROWTH: BlockStateDefinition = BlockStateDefinition::new_int("growth", 0, 7);
pub const HANGING: BlockStateDefinition = BlockStateDefinition::new_bool("hanging", false);
pub const HEAD_PIECE_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("head_piece_bit", false);
pub const HEIGHT: BlockStateDefinition = BlockStateDefinition::new_int("height", 0, 7);
pub const HONEY_LEVEL: BlockStateDefinition = BlockStateDefinition::new_int("honey_level", 0, 5);
pub const HUGE_MUSHROOM_BITS: BlockStateDefinition =
    BlockStateDefinition::new_int("huge_mushroom_bits", 0, 15);
pub const IN_WALL_BIT: BlockStateDefinition = BlockStateDefinition::new_bool("in_wall_bit", false);
pub const INFINIBURN_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("infiniburn_bit", false);
pub const ITEM_FRAME_MAP_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("item_frame_map_bit", false);
pub const ITEM_FRAME_PHOTO_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("item_frame_photo_bit", false);
pub const KELP_AGE: BlockStateDefinition = BlockStateDefinition::new_int("kelp_age", 0, 25);
pub const LEVER_DIRECTION: BlockStateDefinition =
    BlockStateDefinition::new_enum("lever_direction", LeverDirection::VARIANTS);
pub const LIQUID_DEPTH: BlockStateDefinition = BlockStateDefinition::new_int("liquid_depth", 0, 15);
pub const LIT: BlockStateDefinition = BlockStateDefinition::new_bool("lit", false);
pub const MINECRAFT_BLOCK_FACE: BlockStateDefinition =
    BlockStateDefinition::new_enum("minecraft:block_face", BlockFace::VARIANTS);
pub const MINECRAFT_CARDINAL_DIRECTION: BlockStateDefinition = BlockStateDefinition::new_enum(
    "minecraft:cardinal_direction",
    MinecraftCardinalDirection::VARIANTS,
);
pub const MINECRAFT_FACING_DIRECTION: BlockStateDefinition =
    BlockStateDefinition::new_enum("minecraft:facing_direction", BlockFace::VARIANTS);
pub const MINECRAFT_VERTICAL_HALF: BlockStateDefinition =
    BlockStateDefinition::new_enum("minecraft:vertical_half", MinecraftVerticalHalf::VARIANTS);
pub const MOISTURIZED_AMOUNT: BlockStateDefinition =
    BlockStateDefinition::new_int("moisturized_amount", 0, 7);
pub const MONSTER_EGG_STONE_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("monster_egg_stone_type", MonsterEggStoneType::VARIANTS);
pub const MULTI_FACE_DIRECTION_BITS: BlockStateDefinition =
    BlockStateDefinition::new_int("multi_face_direction_bits", 0, 63);
pub const NEW_LEAF_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("new_leaf_type", NewLeafType::VARIANTS);
pub const OCCUPIED_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("occupied_bit", false);
pub const OLD_LEAF_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("old_leaf_type", OldLeafType::VARIANTS);
pub const OPEN_BIT: BlockStateDefinition = BlockStateDefinition::new_bool("open_bit", false);
pub const ORIENTATION: BlockStateDefinition =
    BlockStateDefinition::new_enum("orientation", Orientation::VARIANTS);
pub const OUTPUT_LIT_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("output_lit_bit", false);
pub const OUTPUT_SUBTRACT_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("output_subtract_bit", false);
pub const PERSISTENT_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("persistent_bit", false);
pub const PILLAR_AXIS: BlockStateDefinition =
    BlockStateDefinition::new_enum("pillar_axis", Axis::VARIANTS);
pub const PORTAL_AXIS: BlockStateDefinition =
    BlockStateDefinition::new_enum("portal_axis", PortalAxis::VARIANTS);
pub const POWERED_BIT: BlockStateDefinition = BlockStateDefinition::new_bool("powered_bit", false);
pub const PRISMARINE_BLOCK_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("prismarine_block_type", PrismarineBlockType::VARIANTS);
pub const PROPAGULE_STAGE: BlockStateDefinition =
    BlockStateDefinition::new_int("propagule_stage", 0, 4);
pub const RAIL_DATA_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("rail_data_bit", false);
pub const RAIL_DIRECTION_10: BlockStateDefinition =
    BlockStateDefinition::new_int("rail_direction_10", 0, 9);
pub const RAIL_DIRECTION_6: BlockStateDefinition =
    BlockStateDefinition::new_int("rail_direction_6", 0, 5);
pub const REDSTONE_SIGNAL: BlockStateDefinition =
    BlockStateDefinition::new_int("redstone_signal", 0, 15);
pub const REPEATER_DELAY: BlockStateDefinition =
    BlockStateDefinition::new_int("repeater_delay", 0, 3);
pub const RESPAWN_ANCHOR_CHARGE: BlockStateDefinition =
    BlockStateDefinition::new_int("respawn_anchor_charge", 0, 4);
pub const ROTATION: BlockStateDefinition = BlockStateDefinition::new_int("rotation", 0, 3);
pub const SAND_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("sand_type", SandType::VARIANTS);
pub const SCULK_SENSOR_PHASE: BlockStateDefinition =
    BlockStateDefinition::new_int("sculk_sensor_phase", 0, 2);
pub const SEA_GRASS_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("sea_grass_type", SeaGrassType::VARIANTS);
pub const SPONGE_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("sponge_type", SpongeType::VARIANTS);
pub const STABILITY: BlockStateDefinition = BlockStateDefinition::new_int("stability", 0, 7);
pub const STABILITY_CHECK: BlockStateDefinition =
    BlockStateDefinition::new_bool("stability_check", false);
pub const STONE_BRICK_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("stone_brick_type", StoneBrickType::VARIANTS);
pub const STONE_SLAB_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("stone_slab_type", StoneSlabType::VARIANTS);
pub const STONE_SLAB_TYPE_2: BlockStateDefinition =
    BlockStateDefinition::new_enum("stone_slab_type_2", StoneSlabType2::VARIANTS);
pub const STONE_SLAB_TYPE_3: BlockStateDefinition =
    BlockStateDefinition::new_enum("stone_slab_type_3", StoneSlabType3::VARIANTS);
pub const STONE_SLAB_TYPE_4: BlockStateDefinition =
    BlockStateDefinition::new_enum("stone_slab_type_4", StoneSlabType4::VARIANTS);
pub const STRIPPED_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("stripped_bit", false);
pub const STRUCTURE_BLOCK_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("structure_block_type", StructureBlockType::VARIANTS);
pub const STRUCTURE_VOID_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("structure_void_type", StructureVoidType::VARIANTS);
pub const SUSPENDED_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("suspended_bit", false);
pub const TALL_GRASS_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("tall_grass_type", TallGrassType::VARIANTS);
pub const TOGGLE_BIT: BlockStateDefinition = BlockStateDefinition::new_bool("toggle_bit", false);
pub const TORCH_FACING_DIRECTION: BlockStateDefinition =
    BlockStateDefinition::new_enum("torch_facing_direction", TorchFacingDirection::VARIANTS);
pub const TRIGGERED_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("triggered_bit", false);
pub const TURTLE_EGG_COUNT: BlockStateDefinition =
    BlockStateDefinition::new_enum("turtle_egg_count", TurtleEggCount::VARIANTS);
pub const TWISTING_VINES_AGE: BlockStateDefinition =
    BlockStateDefinition::new_int("twisting_vines_age", 0, 25);
pub const UPDATE_BIT: BlockStateDefinition = BlockStateDefinition::new_bool("update_bit", false);
pub const UPPER_BLOCK_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("upper_block_bit", false);
pub const UPSIDE_DOWN_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("upside_down_bit", false);
pub const VINE_DIRECTION_BITS: BlockStateDefinition =
    BlockStateDefinition::new_int("vine_direction_bits", 0, 15);
pub const WALL_BLOCK_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("wall_block_type", WallBlockType::VARIANTS);
pub const WALL_CONNECTION_TYPE_EAST: BlockStateDefinition =
    BlockStateDefinition::new_enum("wall_connection_type_east", WallConnectionType::VARIANTS);
pub const WALL_CONNECTION_TYPE_NORTH: BlockStateDefinition =
    BlockStateDefinition::new_enum("wall_connection_type_north", WallConnectionType::VARIANTS);
pub const WALL_CONNECTION_TYPE_SOUTH: BlockStateDefinition =
    BlockStateDefinition::new_enum("wall_connection_type_south", WallConnectionType::VARIANTS);
pub const WALL_CONNECTION_TYPE_WEST: BlockStateDefinition =
    BlockStateDefinition::new_enum("wall_connection_type_west", WallConnectionType::VARIANTS);
pub const PALE_MOSS_CARPET_SIDE_EAST: BlockStateDefinition =
    BlockStateDefinition::new_enum("pale_moss_carpet_side_east", PaleMossCarpetSide::VARIANTS);
pub const PALE_MOSS_CARPET_SIDE_NORTH: BlockStateDefinition =
    BlockStateDefinition::new_enum("pale_moss_carpet_side_north", PaleMossCarpetSide::VARIANTS);
pub const PALE_MOSS_CARPET_SIDE_SOUTH: BlockStateDefinition =
    BlockStateDefinition::new_enum("pale_moss_carpet_side_south", PaleMossCarpetSide::VARIANTS);
pub const PALE_MOSS_CARPET_SIDE_WEST: BlockStateDefinition =
    BlockStateDefinition::new_enum("pale_moss_carpet_side_west", PaleMossCarpetSide::VARIANTS);
pub const TIP: BlockStateDefinition = BlockStateDefinition::new_bool("tip", false);
pub const NATURAL: BlockStateDefinition = BlockStateDefinition::new_bool("natural", false);
pub const WALL_POST_BIT: BlockStateDefinition =
    BlockStateDefinition::new_bool("wall_post_bit", false);
pub const WEEPING_VINES_AGE: BlockStateDefinition =
    BlockStateDefinition::new_int("weeping_vines_age", 0, 25);
pub const WEIRDO_DIRECTION: BlockStateDefinition =
    BlockStateDefinition::new_int("weirdo_direction", 0, 3);
pub const WOOD_TYPE: BlockStateDefinition =
    BlockStateDefinition::new_enum("wood_type", WoodType::VARIANTS);
pub const TRIAL_SPAWNER_STATE: BlockStateDefinition =
    BlockStateDefinition::new_int("trial_spawner_state", 0, 5);
pub const VAULT_STATE: BlockStateDefinition =
    BlockStateDefinition::new_enum("vault_state", VaultState::VARIANTS);
pub const OMINOUS: BlockStateDefinition = BlockStateDefinition::new_bool("ominous", false);
