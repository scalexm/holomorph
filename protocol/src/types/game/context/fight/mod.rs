use crate::types::game::character::alignment::ActorAlignmentInformations;
use crate::types::game::character::status::PlayerStatus;
use crate::types::game::context::roleplay::BasicAllianceInformations;
use crate::types::game::context::roleplay::BasicGuildInformations;
use crate::types::game::context::GameContextActorInformations;
use crate::variants::FightResultAdditionalDataVariant;
use crate::variants::FightTeamInformationsVariant;
use crate::variants::FightTeamMemberInformationsVariant;
use crate::variants::GameFightMinimalStatsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 151)]
pub struct GameFightAIInformations<'a> {
    pub base: GameFightFighterInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 456)]
pub struct GameFightFighterNamedLightInformations<'a> {
    pub base: GameFightFighterLightInformations<'a>,
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 50)]
pub struct GameFightMutantInformations<'a> {
    pub base: GameFightFighterNamedInformations<'a>,
    pub power_level: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 48)]
pub struct GameFightTaxCollectorInformations<'a> {
    pub base: GameFightAIInformations<'a>,
    #[protocol(var)]
    pub first_name_id: u16,
    #[protocol(var)]
    pub last_name_id: u16,
    pub level: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 203)]
pub struct GameFightMonsterWithAlignmentInformations<'a> {
    pub base: GameFightMonsterInformations<'a>,
    pub alignment_infos: ActorAlignmentInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 439)]
pub struct FightAllianceTeamInformations<'a> {
    pub base: FightTeamInformations<'a>,
    pub relation: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 116)]
pub struct AbstractFightTeamInformations<'a> {
    pub team_id: u8,
    pub leader_id: f64,
    pub team_side: i8,
    pub team_type_id: u8,
    pub nb_waves: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 44)]
pub struct FightTeamMemberInformations<'a> {
    pub id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 364)]
pub struct GameFightResumeSlaveInfo<'a> {
    pub slave_id: f64,
    pub spell_cooldowns: std::borrow::Cow<'a, [GameFightSpellCooldown<'a>]>,
    pub summon_count: u8,
    pub bomb_count: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 115)]
pub struct FightTeamLightInformations<'a> {
    pub base: AbstractFightTeamInformations<'a>,
    #[protocol(flag)]
    pub has_friend: bool,
    #[protocol(flag)]
    pub has_guild_member: bool,
    #[protocol(flag)]
    pub has_alliance_member: bool,
    #[protocol(flag)]
    pub has_group_member: bool,
    #[protocol(flag)]
    pub has_my_tax_collector: bool,
    pub team_members_count: u8,
    #[protocol(var)]
    pub mean_level: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 216)]
pub struct FightResultMutantListEntry<'a> {
    pub base: FightResultFighterListEntry<'a>,
    #[protocol(var)]
    pub level: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 13)]
pub struct FightTeamMemberCharacterInformations<'a> {
    pub base: FightTeamMemberInformations<'a>,
    pub name: &'a str,
    #[protocol(var)]
    pub level: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 360)]
pub struct GameFightMinimalStatsPreparation<'a> {
    pub base: GameFightMinimalStats<'a>,
    #[protocol(var)]
    pub initiative: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 413)]
pub struct GameFightFighterLightInformations<'a> {
    #[protocol(flag)]
    pub sex: bool,
    #[protocol(flag)]
    pub alive: bool,
    pub id: f64,
    pub wave: u8,
    #[protocol(var)]
    pub level: u16,
    pub breed: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 46)]
pub struct GameFightCharacterInformations<'a> {
    pub base: GameFightFighterNamedInformations<'a>,
    #[protocol(var)]
    pub level: u16,
    pub alignment_infos: ActorAlignmentInformations<'a>,
    pub breed: i8,
    pub sex: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 41)]
pub struct FightLoot<'a> {
    #[protocol(var_contents)]
    pub objects: std::borrow::Cow<'a, [u32]>,
    #[protocol(var)]
    pub kamas: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 84)]
pub struct FightResultTaxCollectorListEntry<'a> {
    pub base: FightResultFighterListEntry<'a>,
    pub level: u8,
    pub guild_info: BasicGuildInformations<'a>,
    pub experience_for_guild: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 177)]
pub struct FightTeamMemberTaxCollectorInformations<'a> {
    pub base: FightTeamMemberInformations<'a>,
    #[protocol(var)]
    pub first_name_id: u16,
    #[protocol(var)]
    pub last_name_id: u16,
    pub level: u8,
    #[protocol(var)]
    pub guild_id: u32,
    pub uid: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 158)]
pub struct GameFightFighterNamedInformations<'a> {
    pub base: GameFightFighterInformations<'a>,
    pub name: &'a str,
    pub status: PlayerStatus<'a>,
    #[protocol(var)]
    pub league_id: i16,
    pub ladder_position: i32,
    pub hidden_in_prefight: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 143)]
pub struct GameFightFighterInformations<'a> {
    pub base: GameContextActorInformations<'a>,
    pub team_id: u8,
    pub wave: u8,
    pub alive: bool,
    pub stats: GameFightMinimalStatsVariant<'a>,
    #[protocol(var_contents)]
    pub previous_positions: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 549)]
pub struct FightTeamMemberEntityInformation<'a> {
    pub base: FightTeamMemberInformations<'a>,
    pub entity_model_id: u8,
    #[protocol(var)]
    pub level: u16,
    pub master_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 455)]
pub struct GameFightFighterMonsterLightInformations<'a> {
    pub base: GameFightFighterLightInformations<'a>,
    #[protocol(var)]
    pub creature_generic_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 513)]
pub struct FightStartingPositions<'a> {
    #[protocol(var_contents)]
    pub positions_for_challengers: std::borrow::Cow<'a, [u16]>,
    #[protocol(var_contents)]
    pub positions_for_defenders: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 205)]
pub struct GameFightSpellCooldown<'a> {
    pub spell_id: i32,
    pub cooldown: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 117)]
pub struct FightExternalInformations<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub fight_type: u8,
    pub fight_start: u32,
    pub fight_spectator_locked: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 189)]
pub struct FightResultFighterListEntry<'a> {
    pub base: FightResultListEntry<'a>,
    pub id: f64,
    pub alive: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 31)]
pub struct GameFightMinimalStats<'a> {
    #[protocol(var)]
    pub life_points: u32,
    #[protocol(var)]
    pub max_life_points: u32,
    #[protocol(var)]
    pub base_max_life_points: u32,
    #[protocol(var)]
    pub permanent_damage_percent: u32,
    #[protocol(var)]
    pub shield_points: u32,
    #[protocol(var)]
    pub action_points: i16,
    #[protocol(var)]
    pub max_action_points: i16,
    #[protocol(var)]
    pub movement_points: i16,
    #[protocol(var)]
    pub max_movement_points: i16,
    pub summoner: f64,
    pub summoned: bool,
    #[protocol(var)]
    pub neutral_element_resist_percent: i16,
    #[protocol(var)]
    pub earth_element_resist_percent: i16,
    #[protocol(var)]
    pub water_element_resist_percent: i16,
    #[protocol(var)]
    pub air_element_resist_percent: i16,
    #[protocol(var)]
    pub fire_element_resist_percent: i16,
    #[protocol(var)]
    pub neutral_element_reduction: i16,
    #[protocol(var)]
    pub earth_element_reduction: i16,
    #[protocol(var)]
    pub water_element_reduction: i16,
    #[protocol(var)]
    pub air_element_reduction: i16,
    #[protocol(var)]
    pub fire_element_reduction: i16,
    #[protocol(var)]
    pub critical_damage_fixed_resist: i16,
    #[protocol(var)]
    pub push_damage_fixed_resist: i16,
    #[protocol(var)]
    pub pvp_neutral_element_resist_percent: i16,
    #[protocol(var)]
    pub pvp_earth_element_resist_percent: i16,
    #[protocol(var)]
    pub pvp_water_element_resist_percent: i16,
    #[protocol(var)]
    pub pvp_air_element_resist_percent: i16,
    #[protocol(var)]
    pub pvp_fire_element_resist_percent: i16,
    #[protocol(var)]
    pub pvp_neutral_element_reduction: i16,
    #[protocol(var)]
    pub pvp_earth_element_reduction: i16,
    #[protocol(var)]
    pub pvp_water_element_reduction: i16,
    #[protocol(var)]
    pub pvp_air_element_reduction: i16,
    #[protocol(var)]
    pub pvp_fire_element_reduction: i16,
    #[protocol(var)]
    pub dodge_pa_lost_probability: u16,
    #[protocol(var)]
    pub dodge_pm_lost_probability: u16,
    #[protocol(var)]
    pub tackle_block: i16,
    #[protocol(var)]
    pub tackle_evade: i16,
    #[protocol(var)]
    pub fixed_damage_reflection: i16,
    pub invisibility_state: u8,
    #[protocol(var)]
    pub melee_damage_received_percent: u16,
    #[protocol(var)]
    pub ranged_damage_received_percent: u16,
    #[protocol(var)]
    pub weapon_damage_received_percent: u16,
    #[protocol(var)]
    pub spell_damage_received_percent: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 33)]
pub struct FightTeamInformations<'a> {
    pub base: AbstractFightTeamInformations<'a>,
    pub team_members: std::borrow::Cow<'a, [FightTeamMemberInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 24)]
pub struct FightResultPlayerListEntry<'a> {
    pub base: FightResultFighterListEntry<'a>,
    #[protocol(var)]
    pub level: u16,
    pub additional: std::borrow::Cow<'a, [FightResultAdditionalDataVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 191)]
pub struct FightResultAdditionalData<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6)]
pub struct FightTeamMemberMonsterInformations<'a> {
    pub base: FightTeamMemberInformations<'a>,
    pub monster_id: i32,
    pub grade: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 457)]
pub struct GameFightFighterTaxCollectorLightInformations<'a> {
    pub base: GameFightFighterLightInformations<'a>,
    #[protocol(var)]
    pub first_name_id: u16,
    #[protocol(var)]
    pub last_name_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 426)]
pub struct FightTeamMemberWithAllianceCharacterInformations<'a> {
    pub base: FightTeamMemberCharacterInformations<'a>,
    pub alliance_infos: BasicAllianceInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 190)]
pub struct FightResultPvpData<'a> {
    pub base: FightResultAdditionalData<'a>,
    pub grade: u8,
    #[protocol(var)]
    pub min_honor_for_grade: u16,
    #[protocol(var)]
    pub max_honor_for_grade: u16,
    #[protocol(var)]
    pub honor: u16,
    #[protocol(var)]
    pub honor_delta: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 29)]
pub struct GameFightMonsterInformations<'a> {
    pub base: GameFightAIInformations<'a>,
    #[protocol(var)]
    pub creature_generic_id: u16,
    pub creature_grade: u8,
    pub creature_level: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 20)]
pub struct FightOptionsInformations<'a> {
    #[protocol(flag)]
    pub is_secret: bool,
    #[protocol(flag)]
    pub is_restricted_to_party_only: bool,
    #[protocol(flag)]
    pub is_closed: bool,
    #[protocol(flag)]
    pub is_asking_for_help: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 548)]
pub struct GameFightFighterEntityLightInformation<'a> {
    pub base: GameFightFighterLightInformations<'a>,
    pub entity_model_id: u8,
    pub master_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 192)]
pub struct FightResultExperienceData<'a> {
    pub base: FightResultAdditionalData<'a>,
    #[protocol(flag)]
    pub show_experience: bool,
    #[protocol(flag)]
    pub show_experience_level_floor: bool,
    #[protocol(flag)]
    pub show_experience_next_level_floor: bool,
    #[protocol(flag)]
    pub show_experience_fight_delta: bool,
    #[protocol(flag)]
    pub show_experience_for_guild: bool,
    #[protocol(flag)]
    pub show_experience_for_mount: bool,
    #[protocol(flag)]
    pub is_incarnation_experience: bool,
    #[protocol(var)]
    pub experience: u64,
    #[protocol(var)]
    pub experience_level_floor: u64,
    #[protocol(var)]
    pub experience_next_level_floor: u64,
    #[protocol(var)]
    pub experience_fight_delta: u64,
    #[protocol(var)]
    pub experience_for_guild: u64,
    #[protocol(var)]
    pub experience_for_mount: u64,
    pub reroll_experience_mul: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 16)]
pub struct FightResultListEntry<'a> {
    #[protocol(var)]
    pub outcome: u16,
    pub wave: u8,
    pub rewards: FightLoot<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 43)]
pub struct FightCommonInformations<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub fight_type: u8,
    pub fight_teams: std::borrow::Cow<'a, [FightTeamInformationsVariant<'a>]>,
    #[protocol(var_contents)]
    pub fight_teams_positions: std::borrow::Cow<'a, [u16]>,
    pub fight_teams_options: std::borrow::Cow<'a, [FightOptionsInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 551)]
pub struct GameFightEntityInformation<'a> {
    pub base: GameFightFighterInformations<'a>,
    pub entity_model_id: u8,
    #[protocol(var)]
    pub level: u16,
    pub master_id: f64,
}
