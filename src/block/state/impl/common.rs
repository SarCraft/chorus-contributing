use crate::block::state::block_state::{BoolBlockState, EnumBlockState, IntBlockState};
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

pub const ACTIVE: BoolBlockState = BoolBlockState::from("active");
pub const AGE_16: IntBlockState = IntBlockState::from("age", 0, 15);
pub const AGE_6: IntBlockState = IntBlockState::from("age", 0, 5);
pub const AGE_4: IntBlockState = IntBlockState::from("age", 0, 3);
pub const AGE_3: IntBlockState = IntBlockState::from("age", 0, 2);
pub const AGE_BIT: BoolBlockState = BoolBlockState::from("age_bit");
pub const ALLOW_UNDERWATER_BIT: BoolBlockState = BoolBlockState::from("allow_underwater_bit");
pub const ATTACHED_BIT: BoolBlockState = BoolBlockState::from("attached_bit");
pub const ATTACHMENT: EnumBlockState = EnumBlockState::from("attachment", Attachment::VARIANTS);
pub const BAMBOO_LEAF_SIZE: EnumBlockState =
    EnumBlockState::from("bamboo_leaf_size", BambooLeafSize::VARIANTS);
pub const BAMBOO_STALK_THICKNESS: EnumBlockState =
    EnumBlockState::from("bamboo_stalk_thickness", BambooStalkThickness::VARIANTS);
pub const BIG_DRIPLEAF_HEAD: BoolBlockState = BoolBlockState::from("big_dripleaf_head");
pub const BIG_DRIPLEAF_TILT: EnumBlockState =
    EnumBlockState::from("big_dripleaf_tilt", BigDripleafTilt::VARIANTS);
pub const BITE_COUNTER: IntBlockState = IntBlockState::from("bite_counter", 0, 6);
pub const BLOCK_LIGHT_LEVEL: IntBlockState = IntBlockState::from("block_light_level", 0, 15);
pub const BLOOM: BoolBlockState = BoolBlockState::from("bloom");
pub const BOOKS_STORED: IntBlockState = IntBlockState::from("books_stored", 0, 63);
pub const BREWING_STAND_SLOT_A_BIT: BoolBlockState =
    BoolBlockState::from("brewing_stand_slot_a_bit");
pub const BREWING_STAND_SLOT_B_BIT: BoolBlockState =
    BoolBlockState::from("brewing_stand_slot_b_bit");
pub const BREWING_STAND_SLOT_C_BIT: BoolBlockState =
    BoolBlockState::from("brewing_stand_slot_c_bit");
pub const BRUSHED_PROGRESS: IntBlockState = IntBlockState::from("brushed_progress", 0, 3);
pub const BUTTON_PRESSED_BIT: BoolBlockState = BoolBlockState::from("button_pressed_bit");
pub const CAN_SUMMON: BoolBlockState = BoolBlockState::from("can_summon");
pub const CANDLES: IntBlockState = IntBlockState::from("candles", 0, 3);
pub const CAULDRON_LIQUID: EnumBlockState =
    EnumBlockState::from("cauldron_liquid", CauldronLiquid::VARIANTS);
pub const CHEMISTRY_TABLE_TYPE: EnumBlockState =
    EnumBlockState::from("chemistry_table_type", ChemistryTableType::VARIANTS);
pub const CHISEL_TYPE: EnumBlockState = EnumBlockState::from("chisel_type", ChiselType::VARIANTS);
pub const CLUSTER_COUNT: IntBlockState = IntBlockState::from("cluster_count", 0, 3);
pub const COLOR: EnumBlockState = EnumBlockState::from("color", Color::VARIANTS);
pub const COLOR_BIT: BoolBlockState = BoolBlockState::from("color_bit");
pub const COMPOSTER_FILL_LEVEL: IntBlockState = IntBlockState::from("composter_fill_level", 0, 8);
pub const CONDITIONAL_BIT: BoolBlockState = BoolBlockState::from("conditional_bit");
pub const CORAL_DIRECTION: IntBlockState = IntBlockState::from("coral_direction", 0, 3);
pub const CORAL_FAN_DIRECTION: IntBlockState = IntBlockState::from("coral_fan_direction", 0, 1);
pub const CORAL_HANG_TYPE_BIT: BoolBlockState = BoolBlockState::from("coral_hang_type_bit");
pub const COVERED_BIT: BoolBlockState = BoolBlockState::from("covered_bit");
pub const CRACKED_STATE: EnumBlockState =
    EnumBlockState::from("cracked_state", CrackedState::VARIANTS);
pub const CRAFTING: BoolBlockState = BoolBlockState::from("crafting");
pub const CREAKING_HEART_STATE: EnumBlockState =
    EnumBlockState::from("creaking_heart_state", CreakingHeartState::VARIANTS);
pub const DAMAGE: EnumBlockState = EnumBlockState::from("damage", Damage::VARIANTS);
pub const DEAD_BIT: BoolBlockState = BoolBlockState::from("dead_bit");
pub const DEPRECATED: IntBlockState = IntBlockState::from("deprecated", 0, 3);
pub const DIRECTION: IntBlockState = IntBlockState::from("direction", 0, 3);
pub const DIRT_TYPE: EnumBlockState = EnumBlockState::from("dirt_type", DirtType::VARIANTS);
pub const DISARMED_BIT: BoolBlockState = BoolBlockState::from("disarmed_bit");
pub const DOOR_HINGE_BIT: BoolBlockState = BoolBlockState::from("door_hinge_bit");
pub const DOUBLE_PLANT_TYPE: EnumBlockState =
    EnumBlockState::from("double_plant_type", DoublePlantType::VARIANTS);
pub const DRAG_DOWN: BoolBlockState = BoolBlockState::from("drag_down");
pub const DRIPSTONE_THICKNESS: EnumBlockState =
    EnumBlockState::from("dripstone_thickness", DripstoneThickness::VARIANTS);
pub const END_PORTAL_EYE_BIT: BoolBlockState = BoolBlockState::from("end_portal_eye_bit");
pub const EXPLODE_BIT: BoolBlockState = BoolBlockState::from("explode_bit");
pub const EXTINGUISHED: BoolBlockState = BoolBlockState::from("extinguished");
pub const CORAL_COLOR: EnumBlockState = EnumBlockState::from("coral_color", CoralColor::VARIANTS);
pub const FACING_DIRECTION: IntBlockState = IntBlockState::from("facing_direction", 0, 5);
pub const FILL_LEVEL: IntBlockState = IntBlockState::from("fill_level", 0, 6);
pub const GROUND_SIGN_DIRECTION: IntBlockState =
    IntBlockState::from("ground_sign_direction", 0, 15);
pub const GROWING_PLANT_AGE: IntBlockState = IntBlockState::from("growing_plant_age", 0, 25);
pub const GROWTH: IntBlockState = IntBlockState::from("growth", 0, 7);
pub const HANGING: BoolBlockState = BoolBlockState::from("hanging");
pub const HEAD_PIECE_BIT: BoolBlockState = BoolBlockState::from("head_piece_bit");
pub const HEIGHT: IntBlockState = IntBlockState::from("height", 0, 7);
pub const HONEY_LEVEL: IntBlockState = IntBlockState::from("honey_level", 0, 5);
pub const HUGE_MUSHROOM_BITS: IntBlockState = IntBlockState::from("huge_mushroom_bits", 0, 15);
pub const IN_WALL_BIT: BoolBlockState = BoolBlockState::from("in_wall_bit");
pub const INFINIBURN_BIT: BoolBlockState = BoolBlockState::from("infiniburn_bit");
pub const ITEM_FRAME_MAP_BIT: BoolBlockState = BoolBlockState::from("item_frame_map_bit");
pub const ITEM_FRAME_PHOTO_BIT: BoolBlockState = BoolBlockState::from("item_frame_photo_bit");
pub const KELP_AGE: IntBlockState = IntBlockState::from("kelp_age", 0, 25);
pub const LEVER_DIRECTION: EnumBlockState =
    EnumBlockState::from("lever_direction", LeverDirection::VARIANTS);
pub const LIQUID_DEPTH: IntBlockState = IntBlockState::from("liquid_depth", 0, 15);
pub const LIT: BoolBlockState = BoolBlockState::from("lit");
pub const MINECRAFT_BLOCK_FACE: EnumBlockState =
    EnumBlockState::from("minecraft:block_face", BlockFace::VARIANTS);
pub const MINECRAFT_CARDINAL_DIRECTION: EnumBlockState = EnumBlockState::from(
    "minecraft:cardinal_direction",
    MinecraftCardinalDirection::VARIANTS,
);
pub const MINECRAFT_FACING_DIRECTION: EnumBlockState =
    EnumBlockState::from("minecraft:facing_direction", BlockFace::VARIANTS);
pub const MINECRAFT_VERTICAL_HALF: EnumBlockState =
    EnumBlockState::from("minecraft:vertical_half", MinecraftVerticalHalf::VARIANTS);
pub const MOISTURIZED_AMOUNT: IntBlockState = IntBlockState::from("moisturized_amount", 0, 7);
pub const MONSTER_EGG_STONE_TYPE: EnumBlockState =
    EnumBlockState::from("monster_egg_stone_type", MonsterEggStoneType::VARIANTS);
pub const MULTI_FACE_DIRECTION_BITS: IntBlockState =
    IntBlockState::from("multi_face_direction_bits", 0, 63);
pub const NEW_LEAF_TYPE: EnumBlockState =
    EnumBlockState::from("new_leaf_type", NewLeafType::VARIANTS);
pub const OCCUPIED_BIT: BoolBlockState = BoolBlockState::from("occupied_bit");
pub const OLD_LEAF_TYPE: EnumBlockState =
    EnumBlockState::from("old_leaf_type", OldLeafType::VARIANTS);
pub const OPEN_BIT: BoolBlockState = BoolBlockState::from("open_bit");
pub const ORIENTATION: EnumBlockState = EnumBlockState::from("orientation", Orientation::VARIANTS);
pub const OUTPUT_LIT_BIT: BoolBlockState = BoolBlockState::from("output_lit_bit");
pub const OUTPUT_SUBTRACT_BIT: BoolBlockState = BoolBlockState::from("output_subtract_bit");
pub const PERSISTENT_BIT: BoolBlockState = BoolBlockState::from("persistent_bit");
pub const PILLAR_AXIS: EnumBlockState = EnumBlockState::from("pillar_axis", Axis::VARIANTS);
pub const PORTAL_AXIS: EnumBlockState = EnumBlockState::from("portal_axis", PortalAxis::VARIANTS);
pub const POWERED_BIT: BoolBlockState = BoolBlockState::from("powered_bit");
pub const PRISMARINE_BLOCK_TYPE: EnumBlockState =
    EnumBlockState::from("prismarine_block_type", PrismarineBlockType::VARIANTS);
pub const PROPAGULE_STAGE: IntBlockState = IntBlockState::from("propagule_stage", 0, 4);
pub const RAIL_DATA_BIT: BoolBlockState = BoolBlockState::from("rail_data_bit");
pub const RAIL_DIRECTION_10: IntBlockState = IntBlockState::from("rail_direction_10", 0, 9);
pub const RAIL_DIRECTION_6: IntBlockState = IntBlockState::from("rail_direction_6", 0, 5);
pub const REDSTONE_SIGNAL: IntBlockState = IntBlockState::from("redstone_signal", 0, 15);
pub const REPEATER_DELAY: IntBlockState = IntBlockState::from("repeater_delay", 0, 3);
pub const RESPAWN_ANCHOR_CHARGE: IntBlockState = IntBlockState::from("respawn_anchor_charge", 0, 4);
pub const ROTATION: IntBlockState = IntBlockState::from("rotation", 0, 3);
pub const SAND_TYPE: EnumBlockState = EnumBlockState::from("sand_type", SandType::VARIANTS);
pub const SCULK_SENSOR_PHASE: IntBlockState = IntBlockState::from("sculk_sensor_phase", 0, 2);
pub const SEA_GRASS_TYPE: EnumBlockState =
    EnumBlockState::from("sea_grass_type", SeaGrassType::VARIANTS);
pub const SPONGE_TYPE: EnumBlockState = EnumBlockState::from("sponge_type", SpongeType::VARIANTS);
pub const STABILITY: IntBlockState = IntBlockState::from("stability", 0, 7);
pub const STABILITY_CHECK: BoolBlockState = BoolBlockState::from("stability_check");
pub const STONE_BRICK_TYPE: EnumBlockState =
    EnumBlockState::from("stone_brick_type", StoneBrickType::VARIANTS);
pub const STONE_SLAB_TYPE: EnumBlockState =
    EnumBlockState::from("stone_slab_type", StoneSlabType::VARIANTS);
pub const STONE_SLAB_TYPE_2: EnumBlockState =
    EnumBlockState::from("stone_slab_type_2", StoneSlabType2::VARIANTS);
pub const STONE_SLAB_TYPE_3: EnumBlockState =
    EnumBlockState::from("stone_slab_type_3", StoneSlabType3::VARIANTS);
pub const STONE_SLAB_TYPE_4: EnumBlockState =
    EnumBlockState::from("stone_slab_type_4", StoneSlabType4::VARIANTS);
pub const STRIPPED_BIT: BoolBlockState = BoolBlockState::from("stripped_bit");
pub const STRUCTURE_BLOCK_TYPE: EnumBlockState =
    EnumBlockState::from("structure_block_type", StructureBlockType::VARIANTS);
pub const STRUCTURE_VOID_TYPE: EnumBlockState =
    EnumBlockState::from("structure_void_type", StructureVoidType::VARIANTS);
pub const SUSPENDED_BIT: BoolBlockState = BoolBlockState::from("suspended_bit");
pub const TALL_GRASS_TYPE: EnumBlockState =
    EnumBlockState::from("tall_grass_type", TallGrassType::VARIANTS);
pub const TOGGLE_BIT: BoolBlockState = BoolBlockState::from("toggle_bit");
pub const TORCH_FACING_DIRECTION: EnumBlockState =
    EnumBlockState::from("torch_facing_direction", TorchFacingDirection::VARIANTS);
pub const TRIGGERED_BIT: BoolBlockState = BoolBlockState::from("triggered_bit");
pub const TURTLE_EGG_COUNT: EnumBlockState =
    EnumBlockState::from("turtle_egg_count", TurtleEggCount::VARIANTS);
pub const TWISTING_VINES_AGE: IntBlockState = IntBlockState::from("twisting_vines_age", 0, 25);
pub const UPDATE_BIT: BoolBlockState = BoolBlockState::from("update_bit");
pub const UPPER_BLOCK_BIT: BoolBlockState = BoolBlockState::from("upper_block_bit");
pub const UPSIDE_DOWN_BIT: BoolBlockState = BoolBlockState::from("upside_down_bit");
pub const VINE_DIRECTION_BITS: IntBlockState = IntBlockState::from("vine_direction_bits", 0, 15);
pub const WALL_BLOCK_TYPE: EnumBlockState =
    EnumBlockState::from("wall_block_type", WallBlockType::VARIANTS);
pub const WALL_CONNECTION_TYPE_EAST: EnumBlockState =
    EnumBlockState::from("wall_connection_type_east", WallConnectionType::VARIANTS);
pub const WALL_CONNECTION_TYPE_NORTH: EnumBlockState =
    EnumBlockState::from("wall_connection_type_north", WallConnectionType::VARIANTS);
pub const WALL_CONNECTION_TYPE_SOUTH: EnumBlockState =
    EnumBlockState::from("wall_connection_type_south", WallConnectionType::VARIANTS);
pub const WALL_CONNECTION_TYPE_WEST: EnumBlockState =
    EnumBlockState::from("wall_connection_type_west", WallConnectionType::VARIANTS);
pub const PALE_MOSS_CARPET_SIDE_EAST: EnumBlockState =
    EnumBlockState::from("pale_moss_carpet_side_east", PaleMossCarpetSide::VARIANTS);
pub const PALE_MOSS_CARPET_SIDE_NORTH: EnumBlockState =
    EnumBlockState::from("pale_moss_carpet_side_north", PaleMossCarpetSide::VARIANTS);
pub const PALE_MOSS_CARPET_SIDE_SOUTH: EnumBlockState =
    EnumBlockState::from("pale_moss_carpet_side_south", PaleMossCarpetSide::VARIANTS);
pub const PALE_MOSS_CARPET_SIDE_WEST: EnumBlockState =
    EnumBlockState::from("pale_moss_carpet_side_west", PaleMossCarpetSide::VARIANTS);
pub const TIP: BoolBlockState = BoolBlockState::from("tip");
pub const NATURAL: BoolBlockState = BoolBlockState::from("natural");
pub const WALL_POST_BIT: BoolBlockState = BoolBlockState::from("wall_post_bit");
pub const WEEPING_VINES_AGE: IntBlockState = IntBlockState::from("weeping_vines_age", 0, 25);
pub const WEIRDO_DIRECTION: IntBlockState = IntBlockState::from("weirdo_direction", 0, 3);
pub const WOOD_TYPE: EnumBlockState = EnumBlockState::from("wood_type", WoodType::VARIANTS);
pub const TRIAL_SPAWNER_STATE: IntBlockState = IntBlockState::from("trial_spawner_state", 0, 5);
pub const VAULT_STATE: EnumBlockState = EnumBlockState::from("vault_state", VaultState::VARIANTS);
pub const OMINOUS: BoolBlockState = BoolBlockState::from("ominous");
