use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SummonerDTO {
    pub account_id: String,
    pub profile_icon_id: i32,
    pub revision_date: i64,
    pub name: String,
    pub id: String,
    pub puuid: String,
    pub summoner_level: i64
}

#[derive(Deserialize, Debug)]
pub struct MatchDto {
    pub metadata: MetadataDto,
    pub info: InfoDto
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataDto {
    pub data_version: String,
    pub match_id: String,
    pub participants: Vec<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InfoDto {
    pub game_creation: i64,
    pub game_duration: i64,
    pub game_end_timestamp: i64,
    pub game_id: i64,
    pub game_mode: String,
    pub game_name: String,
    pub game_start_timestamp: i64,
    pub game_type: String,
    pub game_version: String,
    pub map_id: i32,
    pub participants: Vec<ParticipantDto>,
    pub platform_id: String,
    pub queue_id: i32,
    pub teams: Vec<TeamDto>,
    pub tournament_code: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantDto {
    pub assists: i32,
    pub baron_kills: i32,
    pub bounty_level: i32,
    pub champ_experience: i32,
    pub champ_level: i32,
    pub champion_id: i32,
    pub champion_name: String,
    pub champion_transform: i32,
    pub consumables_purchased: i32,
    pub damage_dealt_to_buildings: i32,
    pub damage_dealt_to_objectives: i32,
    pub damage_dealt_to_turrets: i32,
    pub damage_self_mitigated: i32,
    pub deaths: i32,
    pub detector_wards_placed: i32,
    pub double_kills: i32,
    pub dragon_kills: i32,
    pub first_blood_assist: bool,
    pub first_blood_kill: bool,
    pub first_tower_assist: bool,
    pub first_tower_kill: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub gold_earned: i32,
    pub gold_spent: i32,
    pub individual_position: Position,
    pub inhibitor_kills: i32,
    pub inhibitor_takedowns: i32,
    pub inhibitors_lost: i32,
    pub item0: i32,
    pub item1: i32,
    pub item2: i32,
    pub item3: i32,
    pub item4: i32,
    pub item5: i32,
    pub item6: i32,
    pub items_purchased: i32,
    pub killing_sprees: i32,
    pub kills: i32,
    pub lane: Lane,
    pub largest_critical_strike: i32,
    pub largest_killing_spree: i32,
    pub largest_multi_kill: i32,
    pub longest_time_spent_living: i32,
    pub magic_damage_dealt: i32,
    pub magic_damage_dealt_to_champions: i32,
    pub magic_damage_taken: i32,
    pub neutral_minions_killed: i32,
    pub nexus_kills: i32,
    pub nexus_takedowns: i32,
    pub nexus_lost: i32,
    pub objectives_stolen: i32,
    pub objectives_stolen_assists: i32,
    pub participant_id: i32,
    pub penta_kills: i32,
    pub perks: PerksDto,
    pub physical_damage_dealt: i32,
    pub physical_damage_dealt_to_champions: i32,
    pub physical_damage_taken: i32,
    pub profile_icon: i32,
    pub puuid: String,
    pub quadra_kills: i32,
    pub riot_id_name: String,
    pub riot_id_tagline: String,
    pub role: Role,
    pub sight_wards_bought_in_game: i32,
    pub spell1_casts: i32,
    pub spell2_casts: i32,
    pub spell3_casts: i32,
    pub spell4_casts: i32,
    pub summoner1_casts: i32,
    pub summoner1_id: i32,
    pub summoner2_casts: i32,
    pub summoner2_id: i32,
    pub summoner_id: String,
    pub summoner_level: i32,
    pub summoner_name: String,
    pub team_early_surrendered: bool,
    pub team_id: i32,
    pub team_position: Position,
    #[serde(rename = "timeCCingOthers")]
    pub time_ccing_others: i32,
    pub time_played: i32,
    pub total_damage_dealt: i32,
    pub total_damage_dealt_to_champions: i32,
    pub total_damage_shielded_on_teammates: i32,
    pub total_damage_taken: i32,
    pub total_heal: i32,
    pub total_heals_on_teammates: i32,
    pub total_minions_killed: i32,
    #[serde(rename = "totalTimeCCDealt")]
    pub total_time_cc_dealt: i32,
    pub total_time_spent_dead: i32,
    pub total_units_healed: i32,
    pub triple_kills: i32,
    pub true_damage_dealt: i32,
    pub true_damage_dealt_to_champions: i32,
    pub true_damage_taken: i32,
    pub turret_kills: i32,
    pub turrets_lost: i32,
    pub unreal_kills: i32,
    pub vision_score: i32,
    pub vision_wards_bought_in_game: i32,
    pub wards_killed: i32,
    pub wards_placed: i32,
    pub win: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerksDto {
    pub stat_perks: PerkStatsDto,
    pub styles: Vec<PerkStyleDto>
}

#[derive(Deserialize, Debug)]
pub struct PerkStatsDto {
    pub defense: i32,
    pub flex: i32,
    pub offense: i32
}

#[derive(Deserialize, Debug)]
pub struct PerkStyleDto {
    pub description: String,
    pub selections: Vec<PerkStyleSelectionDto>,
    pub style: i32
}

#[derive(Deserialize, Debug)]
pub struct PerkStyleSelectionDto {
    pub perk: i32,
    pub var1: i32,
    pub var2: i32,
    pub var3: i32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamDto {
    pub bans: Vec<BanDto>,
    pub objectives: ObjectivesDto,
    pub team_id: i32,
    pub win: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BanDto {
    pub champion_id: i32,
    pub pick_turn: i32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObjectivesDto {
    pub baron: ObjectiveDto,
    pub champion: ObjectiveDto,
    pub dragon: ObjectiveDto,
    pub inhibitor: ObjectiveDto,
    pub rift_herald: ObjectiveDto,
    pub tower: ObjectiveDto
}

#[derive(Deserialize, Debug)]
pub struct ObjectiveDto {
    pub first: bool,
    pub kills: i32
}

#[derive(Deserialize, Debug)]
pub struct MatchTimelineDto {
    pub metadata: MetadataDto,
    pub info: MatchTimelineInfo
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfo {
    pub frame_interval: i32,
    pub frames: Vec<MatchTimelineInfoFrame>,
    pub game_id: i64,
    pub participants: Vec<MatchTimelineInfoParticipant>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoParticipant {
    pub participant_id: i32,
    pub puuid: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoFrame {
    pub events: Vec<MatchTimelineInfoFrameEvent>,
    pub participant_frames: MatchTimelineInfoFrameParticipantFrames,
    pub timestamp: i32
}

#[derive(Deserialize, Debug)]
pub struct MatchTimelineInfoFrameParticipantFrames {
    #[serde(rename = "1")]
    pub one: MatchTimelineInfoFrameParticipantFrame,
    #[serde(rename = "2")]
    pub two: MatchTimelineInfoFrameParticipantFrame,
    #[serde(rename = "3")]
    pub three: MatchTimelineInfoFrameParticipantFrame,
    #[serde(rename = "4")]
    pub four: MatchTimelineInfoFrameParticipantFrame,
    #[serde(rename = "5")]
    pub five: MatchTimelineInfoFrameParticipantFrame,
    #[serde(rename = "6")]
    pub six: MatchTimelineInfoFrameParticipantFrame,
    #[serde(rename = "7")]
    pub seven: MatchTimelineInfoFrameParticipantFrame,
    #[serde(rename = "8")]
    pub eight: MatchTimelineInfoFrameParticipantFrame,
    #[serde(rename = "9")]
    pub nine: MatchTimelineInfoFrameParticipantFrame,
    #[serde(rename = "10")]
    pub ten: MatchTimelineInfoFrameParticipantFrame,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoFrameParticipantFrame {
    pub champion_stats: MatchTimelineInfoFrameParticipantFrameChampionStats,
    pub current_gold: i32,
    pub damage_stats: MatchTimelineInfoFrameParticipantFrameDamageStats,
    pub gold_per_second: i32,
    pub jungle_minions_killed: i32,
    pub level: i32,
    pub minions_killed: i32,
    pub participant_id: i32,
    pub position: MatchTimelinPosition,
    pub time_enemy_spent_controlled: i32,
    pub total_gold: i32,
    pub xp: i32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoFrameParticipantFrameChampionStats {
    pub ability_haste: i32,
    pub ability_power: i32,
    pub armor: i32,
    pub armor_pen: i32,
    pub armor_pen_percent: i32,
    pub attack_damage: i32,
    pub attack_speed: i32,
    pub bonus_armor_pen_percent: i32,
    pub bonus_magic_pen_percent: i32,
    pub cc_reduction: i32,
    pub cooldown_reduction: i32,
    pub health: i32,
    pub health_max: i32,
    pub health_regen: i32,
    pub lifesteal: i32,
    pub magic_pen: i32,
    pub magic_pen_percent: i32,
    pub magic_resist: i32,
    pub movement_speed: i32,
    pub omnivamp: i32,
    pub physical_vamp: i32,
    pub power_max: i32,
    pub power_regen: i32,
    pub spell_vamp: i32
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoFrameParticipantFrameDamageStats {
    pub magic_damage_done: i32,
    pub magic_damage_done_to_champions: i32,
    pub magic_damage_taken: i32,
    pub physical_damage_done: i32,
    pub physical_damage_done_to_champions: i32,
    pub physical_damage_taken: i32,
    pub total_damage_done: i32,
    pub total_damage_done_to_champions: i32,
    pub total_damage_taken: i32,
    pub true_damage_done: i32,
    pub true_damage_done_to_champions: i32,
    pub true_damage_taken: i32
}

#[derive(Deserialize, Debug)]
pub struct MatchTimelinPosition {
    pub x: i32,
    pub y: i32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoFrameEvent {
    pub real_timestamp: Option<i64>,
    pub timestamp: i32,
    #[serde(rename = "type")]
    pub type_: EventType,
    pub item_id: Option<i32>,
    pub participant_id: Option<i32>,
    pub level_up_type: Option<String>,
    pub skill_slot: Option<i32>,
    pub creator_id: Option<i32>,
    pub ward_type: Option<String>,
    pub level: Option<i32>,
    pub assisting_participant_ids: Option<Vec<i32>>,
    pub bounty: Option<i32>,
    pub kill_streak_length: Option<i32>,
    pub killed_id: Option<i32>,
    pub position: Option<MatchTimelinPosition>,
    pub victim_damage_dealt: Option<Vec<MatchTimelineInfoFrameEventVictimDamageReceived>>,
    pub victim_damage_received: Option<Vec<MatchTimelineInfoFrameEventVictimDamageReceived>>,
    pub victim_id: Option<i32>,
    pub kill_type: Option<String>,
    pub lane_type: Option<String>,
    pub team_id: Option<i32>,
    pub multi_kill_length: Option<i32>,
    pub monster_type: Option<String>,
    pub monster_sub_type: Option<String>,
    pub building_type: Option<String>,
    pub tower_type: Option<String>,
    pub after_id: Option<i32>,
    pub before_id: Option<i32>,
    pub gold_gain: Option<i32>,
    pub game_id: Option<i64>,
    pub winning_team: Option<i32>,
    pub transform_type: Option<String>,
    pub name: Option<String>,
    pub shutdown_bounty: Option<i32>,
    pub actual_start_time: Option<i64>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineInfoFrameEventVictimDamageReceived {
    pub basic: bool,
    pub magic_damage: i32,
    pub name: String,
    pub participant_id: i32,
    pub physical_damage: i32,
    pub spell_name: String,
    pub spell_slot: i32,
    pub true_damage: i32,
    #[serde(rename = "type")]
    pub type_: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Lane {
    None,
    Top,
    Jungle,
    Middle,
    Bottom
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    None,
    Solo,
    Carry,
    Duo,
    Support
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Position {
    #[serde(rename = "")]
    Unkown,
    #[serde(rename = "Invalid")]
    Invalid,
    Top,
    Jungle,
    Middle,
    Bottom,
    Utility
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    AscendedEvent,
    BuildingKill,
    CapturePoint,
    ChampionKill,
    ChampionSpecialKill,
    ChampionTransform,
    DragonSoulGiven,
    EliteMonsterKill,
    GameEnd,
    ItemDestroyed,
    ItemPurchased,
    ItemSold,
    ItemUndo,
    LevelUp,
    PauseEnd,
    PauseStart,
    SkillLevelUp,
    TurretPlateDestroyed,
    WardKill,
    WardPlaced
}