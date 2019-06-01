use crate::types::game::character::alignment::ActorExtendedAlignmentInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 215)]
pub struct CharacterSpellModification<'a> {
    pub modification_type: u8,
    #[protocol(var)]
    pub spell_id: u16,
    pub value: CharacterBaseCharacteristic<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 4)]
pub struct CharacterBaseCharacteristic<'a> {
    #[protocol(var)]
    pub base: i16,
    #[protocol(var)]
    pub additionnal: i16,
    #[protocol(var)]
    pub objects_and_mount_bonus: i16,
    #[protocol(var)]
    pub align_gift_bonus: i16,
    #[protocol(var)]
    pub context_modif: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 8)]
pub struct CharacterCharacteristicsInformations<'a> {
    #[protocol(var)]
    pub experience: u64,
    #[protocol(var)]
    pub experience_level_floor: u64,
    #[protocol(var)]
    pub experience_next_level_floor: u64,
    #[protocol(var)]
    pub experience_bonus_limit: u64,
    #[protocol(var)]
    pub kamas: u64,
    #[protocol(var)]
    pub stats_points: u16,
    #[protocol(var)]
    pub additionnal_points: u16,
    #[protocol(var)]
    pub spells_points: u16,
    pub alignment_infos: ActorExtendedAlignmentInformations<'a>,
    #[protocol(var)]
    pub life_points: u32,
    #[protocol(var)]
    pub max_life_points: u32,
    #[protocol(var)]
    pub energy_points: u16,
    #[protocol(var)]
    pub max_energy_points: u16,
    #[protocol(var)]
    pub action_points_current: i16,
    #[protocol(var)]
    pub movement_points_current: i16,
    pub initiative: CharacterBaseCharacteristic<'a>,
    pub prospecting: CharacterBaseCharacteristic<'a>,
    pub action_points: CharacterBaseCharacteristic<'a>,
    pub movement_points: CharacterBaseCharacteristic<'a>,
    pub strength: CharacterBaseCharacteristic<'a>,
    pub vitality: CharacterBaseCharacteristic<'a>,
    pub wisdom: CharacterBaseCharacteristic<'a>,
    pub chance: CharacterBaseCharacteristic<'a>,
    pub agility: CharacterBaseCharacteristic<'a>,
    pub intelligence: CharacterBaseCharacteristic<'a>,
    pub range: CharacterBaseCharacteristic<'a>,
    pub summonable_creatures_boost: CharacterBaseCharacteristic<'a>,
    pub reflect: CharacterBaseCharacteristic<'a>,
    pub critical_hit: CharacterBaseCharacteristic<'a>,
    #[protocol(var)]
    pub critical_hit_weapon: u16,
    pub critical_miss: CharacterBaseCharacteristic<'a>,
    pub heal_bonus: CharacterBaseCharacteristic<'a>,
    pub all_damages_bonus: CharacterBaseCharacteristic<'a>,
    pub weapon_damages_bonus_percent: CharacterBaseCharacteristic<'a>,
    pub damages_bonus_percent: CharacterBaseCharacteristic<'a>,
    pub trap_bonus: CharacterBaseCharacteristic<'a>,
    pub trap_bonus_percent: CharacterBaseCharacteristic<'a>,
    pub glyph_bonus_percent: CharacterBaseCharacteristic<'a>,
    pub rune_bonus_percent: CharacterBaseCharacteristic<'a>,
    pub permanent_damage_percent: CharacterBaseCharacteristic<'a>,
    pub tackle_block: CharacterBaseCharacteristic<'a>,
    pub tackle_evade: CharacterBaseCharacteristic<'a>,
    pub pa_attack: CharacterBaseCharacteristic<'a>,
    pub pm_attack: CharacterBaseCharacteristic<'a>,
    pub push_damage_bonus: CharacterBaseCharacteristic<'a>,
    pub critical_damage_bonus: CharacterBaseCharacteristic<'a>,
    pub neutral_damage_bonus: CharacterBaseCharacteristic<'a>,
    pub earth_damage_bonus: CharacterBaseCharacteristic<'a>,
    pub water_damage_bonus: CharacterBaseCharacteristic<'a>,
    pub air_damage_bonus: CharacterBaseCharacteristic<'a>,
    pub fire_damage_bonus: CharacterBaseCharacteristic<'a>,
    pub dodge_pa_lost_probability: CharacterBaseCharacteristic<'a>,
    pub dodge_pm_lost_probability: CharacterBaseCharacteristic<'a>,
    pub neutral_element_resist_percent: CharacterBaseCharacteristic<'a>,
    pub earth_element_resist_percent: CharacterBaseCharacteristic<'a>,
    pub water_element_resist_percent: CharacterBaseCharacteristic<'a>,
    pub air_element_resist_percent: CharacterBaseCharacteristic<'a>,
    pub fire_element_resist_percent: CharacterBaseCharacteristic<'a>,
    pub neutral_element_reduction: CharacterBaseCharacteristic<'a>,
    pub earth_element_reduction: CharacterBaseCharacteristic<'a>,
    pub water_element_reduction: CharacterBaseCharacteristic<'a>,
    pub air_element_reduction: CharacterBaseCharacteristic<'a>,
    pub fire_element_reduction: CharacterBaseCharacteristic<'a>,
    pub push_damage_reduction: CharacterBaseCharacteristic<'a>,
    pub critical_damage_reduction: CharacterBaseCharacteristic<'a>,
    pub pvp_neutral_element_resist_percent: CharacterBaseCharacteristic<'a>,
    pub pvp_earth_element_resist_percent: CharacterBaseCharacteristic<'a>,
    pub pvp_water_element_resist_percent: CharacterBaseCharacteristic<'a>,
    pub pvp_air_element_resist_percent: CharacterBaseCharacteristic<'a>,
    pub pvp_fire_element_resist_percent: CharacterBaseCharacteristic<'a>,
    pub pvp_neutral_element_reduction: CharacterBaseCharacteristic<'a>,
    pub pvp_earth_element_reduction: CharacterBaseCharacteristic<'a>,
    pub pvp_water_element_reduction: CharacterBaseCharacteristic<'a>,
    pub pvp_air_element_reduction: CharacterBaseCharacteristic<'a>,
    pub pvp_fire_element_reduction: CharacterBaseCharacteristic<'a>,
    pub melee_damage_done_percent: CharacterBaseCharacteristic<'a>,
    pub melee_damage_received_percent: CharacterBaseCharacteristic<'a>,
    pub ranged_damage_done_percent: CharacterBaseCharacteristic<'a>,
    pub ranged_damage_received_percent: CharacterBaseCharacteristic<'a>,
    pub weapon_damage_done_percent: CharacterBaseCharacteristic<'a>,
    pub weapon_damage_received_percent: CharacterBaseCharacteristic<'a>,
    pub spell_damage_done_percent: CharacterBaseCharacteristic<'a>,
    pub spell_damage_received_percent: CharacterBaseCharacteristic<'a>,
    pub spell_modifications: std::borrow::Cow<'a, [CharacterSpellModification<'a>]>,
    pub probation_time: u32,
}
