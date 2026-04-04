use crate::config::Config;
use crate::network::handler::PacketReceivedMessage;
use crate::network::session::Session;
use crate::network::session::state::{SessionState, SessionStateChangedMessage};
use crate::player::Player;
use crate::server::ServerState;
use bedrockrs::proto::v662::enums::{
    ChatRestrictionLevel, Difficulty, EditorWorldType, EducationEditionOffer, GamePublishSetting,
    GameType, GeneratorType, PlayStatus, PlayerPermissionLevel, SpawnBiomeType,
};
use bedrockrs::proto::v662::types::{
    ActorRuntimeID, ActorUniqueID, BaseGameVersion, EduSharedUriResource, Experiments,
    NetworkPermissions, SpawnSettings,
};
use bedrockrs::proto::v818::types::SyncedPlayerMovementSettings;
use bedrockrs::proto::v924::types::{GameRuleLegacyData, LevelSettings};
use bedrockrs::proto::v944::packets::StartGamePacket;
use bedrockrs::proto::v944::types::NetworkBlockPosition;
use bedrockrs::proto::{ProtoVersion, ProtoVersionPackets, V944};
use bevy_ecs::change_detection::Res;
use bevy_ecs::message::{MessageReader, MessageWriter};
use bevy_ecs::prelude::{Commands, ParamSet, Query};
use bevy_ecs::system::ResMut;
use tracing::{debug, warn};

pub fn on_enter_setup(
    sessions: Query<&Session>,
    mut server_state: ResMut<ServerState>,
    mut state_reader: MessageReader<SessionStateChangedMessage>,
    mut commands: Commands,
) {
    for ev in state_reader.read() {
        if ev.to != SessionState::Setup {
            continue;
        }

        let Ok(session) = sessions.get(ev.entity) else {
            continue;
        };

        let player = Player::new(server_state.get_runtime_id());

        send_start_game(&player, &session);

        commands.entity(ev.entity).insert(player);
    }
}

fn send_start_game(player: &Player, session: &Session) {
    session.send_immediate(V944::StartGamePacket(StartGamePacket {
        target_actor_id: ActorUniqueID(player.unique_id()),
        target_runtime_id: ActorRuntimeID(player.runtime_id()),
        actor_game_type: GameType::Survival,
        position: Default::default(),
        rotation: Default::default(),
        settings: LevelSettings {
            seed: 0,
            spawn_settings: SpawnSettings {
                spawn_type: SpawnBiomeType::Default,
                user_defined_biome_name: "plains".to_string(),
                dimension: 0,
            },
            generator_type: GeneratorType::Flat,
            game_type: GameType::Survival,
            is_hardcore_enabled: false,
            game_difficulty: Difficulty::Peaceful,
            default_spawn_block_position: NetworkBlockPosition { x: 0, y: 0, z: 0 },
            achievements_disabled: false,
            editor_world_type: EditorWorldType::NonEditor,
            is_created_in_editor: false,
            is_exported_from_editor: false,
            day_cycle_stop_time: 0,
            education_edition_offer: EducationEditionOffer::None,
            education_features_enabled: false,
            education_product_id: "".to_string(),
            rain_level: 0.0,
            lightning_level: 0.0,
            has_confirmed_platform_locked_content: false,
            multiplayer_enabled: false,
            lan_broadcasting_enabled: false,
            xbox_live_broadcast_setting: GamePublishSetting::Public,
            platform_broadcast_setting: GamePublishSetting::Public,
            commands_enabled: false,
            texture_packs_required: false,
            rule_data: GameRuleLegacyData { rules_list: vec![] },
            experiments: Experiments {
                experiments: vec![],
                ever_toggled: false,
            },
            bonus_chest_enabled: false,
            starting_map_enabled: false,
            player_permissions: PlayerPermissionLevel::Member,
            server_chunk_tick_range: 0,
            locked_behaviour_pack: false,
            locked_resource_pack: false,
            from_locked_template: false,
            use_msa_gamer_tags: false,
            from_template: false,
            has_locked_template_settings: false,
            only_spawn_v1_villagers: false,
            persona_disabled: false,
            custom_skins_disabled: false,
            emote_chat_muted: false,
            base_game_version: BaseGameVersion("*".to_string()),
            limited_world_width: 0,
            limited_world_depth: 0,
            nether_type: false,
            edu_shared_uri_resource: EduSharedUriResource {
                button_name: "".to_string(),
                link_uri: "".to_string(),
            },
            override_force_experimental_gameplay: None,
            chat_restriction_level: ChatRestrictionLevel::None,
            disable_player_interactions: false,
        },
        level_id: "".to_string(),
        level_name: "".to_string(),
        template_content_identity: "".to_string(),
        is_trial: false,
        movement_settings: SyncedPlayerMovementSettings {
            rewind_history_size: 0,
            server_authoritative_block_breaking: false,
        },
        current_level_time: 0,
        enchantment_seed: 0,
        block_properties: vec![],
        multiplayer_correlation_id: "".to_string(),
        enable_item_stack_net_manager: false,
        server_version: V944::GAME_VERSION.to_string(),
        player_property_data: Default::default(),
        server_block_type_registry_checksum: 0,
        world_template_id: Default::default(),
        server_enabled_client_side_generation: false,
        block_network_ids_are_hashes: false,
        network_permissions: NetworkPermissions {
            server_auth_sound_enabled: false,
        },
        server_join_information: None,
        server_id: "".to_string(),
        world_id: "".to_string(),
        scenario_id: "".to_string(),
        owner_id: "".to_string(),
    }))
}

pub fn handle_setup(
    mut packet_reader: MessageReader<PacketReceivedMessage>,
    mut state_writer: MessageWriter<SessionStateChangedMessage>,
    mut query: Query<(&Player, &mut Session)>,
) {
    for ev in packet_reader.read() {
        let Ok(mut query) = query.get_mut(ev.entity) else {
            continue;
        };

        match &ev.packet {
            V944::RequestChunkRadiusPacket(packet) => handle_request_chunk_radius(packet),
            V944::SetLocalPlayerAsInitializedPacket(packet) => {
                handle_set_local_player_as_initialized(
                    packet,
                    query.0,
                    &mut query.1,
                    &mut state_writer,
                )
            }
            packet => {
                warn!("unexpected packet received in setup state: {:?}", packet)
            }
        }
    }
}

fn handle_request_chunk_radius(packet: &<V944 as ProtoVersionPackets>::RequestChunkRadiusPacket) {
    debug!("received {:?}", packet);
}

fn handle_set_local_player_as_initialized(
    packet: &<V944 as ProtoVersionPackets>::SetLocalPlayerAsInitializedPacket,
    player: &Player,
    session: &mut Session,
    state_writer: &mut MessageWriter<SessionStateChangedMessage>,
) {
    if packet.player_id.0 != player.runtime_id() {
        warn!(
            "received unexpected player_id {}, expected {}",
            packet.player_id.0,
            player.runtime_id()
        );
        return;
    };

    session.send_play_status(PlayStatus::PlayerSpawn, false);

    session.set_state(SessionState::Play, state_writer);
}
