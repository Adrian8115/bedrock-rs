use bedrockrs_core::int::{LE, VAR};
use bedrockrs_proto_macros::ProtoCodec;

use bedrockrs_shared::world::difficulty::Difficulty;
use bedrockrs_shared::world::gamemode::Gamemode;
use bedrockrs_shared::world::generator_type::GeneratorType;

use crate::types::base_game_version::BaseGameVersion;
use crate::types::block_pos::BlockPos;
use crate::types::chat_restriction_level::ChatRestrictionLevel;
use crate::types::edu_shared_uri_resource::EduSharedResourceUri;
use crate::types::experiments::Experiments;
use crate::types::gamerule::GameRule;
use crate::types::spawn_settings::SpawnSettings;
use bedrockrs_shared::world::editor_world_type::EditorWorldType;

#[derive(ProtoCodec, Debug, Clone)]
pub struct LevelSettings {
    pub seed: LE<u64>,
    pub spawn_settings: SpawnSettings,
    pub generator_type: GeneratorType,
    pub gamemode: Gamemode,
    pub hardcore: bool,
    pub difficulty: Difficulty,
    pub default_spawn_block: BlockPos,
    pub achievements_disabled: bool,
    pub editor_world_type: EditorWorldType,
    pub created_in_editor: bool,
    pub exported_from_editor: bool,
    pub day_cycle_stop_time: VAR<i32>,
    pub education_edition_offer: VAR<i32>,
    pub education_features: bool,
    pub education_product_id: String,
    pub rain_level: LE<f32>,
    pub lightning_level: LE<f32>,
    pub platform_locked_content: bool,
    pub multiplayer_intended: bool,
    pub lan_broadcasting_intended: bool,
    // TODO turn into enum
    pub broadcasting_settings_xbox_live: VAR<i32>,
    pub broadcasting_settings_platform: VAR<i32>,
    pub commands_enabled: bool,
    pub texture_pack_required: bool,
    #[len_repr(VAR::<u32>)]
    pub gamerules: Vec<GameRule>,
    pub experiments: Experiments,
    pub bonus_chest: bool,
    pub start_with_map: bool,
    // TODO turn into enum
    pub player_permission: VAR<i32>,
    pub server_chunk_tick_radius: LE<i32>,
    pub locked_behavior_packs: bool,
    pub locked_resource_packs: bool,
    pub from_locked_template: bool,
    pub msa_gamertags_only: bool,
    pub from_template: bool,
    pub is_template_locked_settings: bool,
    pub only_spawn_v1_villagers: bool,
    pub persona_disabled: bool,
    pub custom_skins_disabled: bool,
    pub emote_chat_muted: bool,
    pub base_game_version: BaseGameVersion,
    pub limited_world_width: LE<i32>,
    pub limited_world_depth: LE<i32>,
    pub new_nether: bool,
    pub edu_shared_uri_resource: EduSharedResourceUri,
    pub force_experimental_gameplay: bool,
    pub chat_restriction_level: ChatRestrictionLevel,
    pub disable_player_interactions: bool,
    pub server_id: String,
    pub world_id: String,
    pub scenario_id: String,
}
