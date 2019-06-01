pub mod breach;
pub mod fight;
pub mod job;
pub mod party;
pub mod quest;
pub mod treasure_hunt;

use crate::types::game::character::alignment::ActorAlignmentInformations;
use crate::types::game::character::restriction::ActorRestrictionsInformations;
use crate::types::game::context::roleplay::quest::GameRolePlayNpcQuestFlag;
use crate::types::game::context::GameContextActorInformations;
use crate::types::game::context::MapCoordinatesExtended;
use crate::types::game::guild::GuildEmblem;
use crate::types::game::look::EntityLook;
use crate::types::game::look::IndexedEntityLook;
use crate::types::game::social::AbstractSocialGroupInfos;
use crate::variants::GroupMonsterStaticInformationsVariant;
use crate::variants::HumanInformationsVariant;
use crate::variants::HumanOptionVariant;
use crate::variants::PortalInformationVariant;
use crate::variants::PrismInformationVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 396)]
pub struct GroupMonsterStaticInformationsWithAlternatives<'a> {
    pub base: GroupMonsterStaticInformations<'a>,
    pub alternatives: std::borrow::Cow<'a, [AlternativeMonstersInGroupLightInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 419)]
pub struct BasicAllianceInformations<'a> {
    pub base: AbstractSocialGroupInfos<'a>,
    #[protocol(var)]
    pub alliance_id: u32,
    pub alliance_tag: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 157)]
pub struct HumanInformations<'a> {
    pub restrictions: ActorRestrictionsInformations<'a>,
    pub sex: bool,
    pub options: std::borrow::Cow<'a, [HumanOptionVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 198)]
pub struct ObjectItemInRolePlay<'a> {
    #[protocol(var)]
    pub cell_id: u16,
    #[protocol(var)]
    pub object_gid: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 497)]
pub struct MonsterBoosts<'a> {
    #[protocol(var)]
    pub id: u32,
    #[protocol(var)]
    pub xp_boost: u16,
    #[protocol(var)]
    pub drop_boost: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 406)]
pub struct HumanOption<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 160)]
pub struct GameRolePlayGroupMonsterInformations<'a> {
    pub base: GameRolePlayActorInformations<'a>,
    #[protocol(flag)]
    pub key_ring_bonus: bool,
    #[protocol(flag)]
    pub has_hardcore_drop: bool,
    #[protocol(flag)]
    pub has_ava_reward_token: bool,
    pub static_infos: GroupMonsterStaticInformationsVariant<'a>,
    pub loot_share: i8,
    pub alignment_side: i8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 464)]
pub struct GameRolePlayGroupMonsterWaveInformations<'a> {
    pub base: GameRolePlayGroupMonsterInformations<'a>,
    pub nb_waves: u8,
    pub alternatives: std::borrow::Cow<'a, [GroupMonsterStaticInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 175)]
pub struct AtlasPointsInformations<'a> {
    pub type_: u8,
    pub coords: std::borrow::Cow<'a, [MapCoordinatesExtended<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 418)]
pub struct BasicNamedAllianceInformations<'a> {
    pub base: BasicAllianceInformations<'a>,
    pub alliance_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 383)]
pub struct GameRolePlayNpcWithQuestInformations<'a> {
    pub base: GameRolePlayNpcInformations<'a>,
    pub quest_flag: GameRolePlayNpcQuestFlag<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3)]
pub struct GameRolePlayMutantInformations<'a> {
    pub base: GameRolePlayHumanoidInformations<'a>,
    #[protocol(var)]
    pub monster_id: u16,
    pub power_level: i8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 161)]
pub struct GameRolePlayPrismInformations<'a> {
    pub base: GameRolePlayActorInformations<'a>,
    pub prism: PrismInformationVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 127)]
pub struct GuildInformations<'a> {
    pub base: BasicGuildInformations<'a>,
    pub guild_emblem: GuildEmblem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 471)]
pub struct GameRolePlayTreasureHintInformations<'a> {
    pub base: GameRolePlayActorInformations<'a>,
    #[protocol(var)]
    pub npc_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 394)]
pub struct AlternativeMonstersInGroupLightInformations<'a> {
    pub player_count: i32,
    pub monsters: std::borrow::Cow<'a, [MonsterInGroupLightInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 159)]
pub struct GameRolePlayHumanoidInformations<'a> {
    pub base: GameRolePlayNamedActorInformations<'a>,
    pub humanoid_info: HumanInformationsVariant<'a>,
    pub account_id: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 407)]
pub struct HumanOptionEmote<'a> {
    pub base: HumanOption<'a>,
    pub emote_id: u8,
    pub emote_start_time: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 144)]
pub struct MonsterInGroupInformations<'a> {
    pub base: MonsterInGroupLightInformations<'a>,
    pub look: EntityLook<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 411)]
pub struct HumanOptionOrnament<'a> {
    pub base: HumanOption<'a>,
    #[protocol(var)]
    pub ornament_id: u16,
    #[protocol(var)]
    pub level: u16,
    #[protocol(var)]
    pub league_id: i16,
    pub ladder_position: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 449)]
pub struct HumanOptionObjectUse<'a> {
    pub base: HumanOption<'a>,
    pub delay_type_id: u8,
    pub delay_end_time: f64,
    #[protocol(var)]
    pub object_gid: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 420)]
pub struct GuildInAllianceInformations<'a> {
    pub base: GuildInformations<'a>,
    pub nb_members: u8,
    pub join_date: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 409)]
pub struct HumanOptionGuild<'a> {
    pub base: HumanOption<'a>,
    pub guild_informations: GuildInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 408)]
pub struct HumanOptionTitle<'a> {
    pub base: HumanOption<'a>,
    #[protocol(var)]
    pub title_id: u16,
    pub title_param: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 467)]
pub struct GameRolePlayPortalInformations<'a> {
    pub base: GameRolePlayActorInformations<'a>,
    pub portal: PortalInformationVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 425)]
pub struct HumanOptionAlliance<'a> {
    pub base: HumanOption<'a>,
    pub alliance_informations: AllianceInformations<'a>,
    pub aggressable: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 154)]
pub struct GameRolePlayNamedActorInformations<'a> {
    pub base: GameRolePlayActorInformations<'a>,
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 365)]
pub struct BasicGuildInformations<'a> {
    pub base: AbstractSocialGroupInfos<'a>,
    #[protocol(var)]
    pub guild_id: u32,
    pub guild_name: &'a str,
    pub guild_level: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 395)]
pub struct MonsterInGroupLightInformations<'a> {
    pub generic_id: i32,
    pub grade: u8,
    pub level: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 141)]
pub struct GameRolePlayActorInformations<'a> {
    pub base: GameContextActorInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 180)]
pub struct GameRolePlayMountInformations<'a> {
    pub base: GameRolePlayNamedActorInformations<'a>,
    pub owner_name: &'a str,
    pub level: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 565)]
pub struct AnomalySubareaInformation<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    #[protocol(var)]
    pub reward_rate: i16,
    pub has_anomaly: bool,
    #[protocol(var)]
    pub anomaly_closing_time: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 156)]
pub struct GameRolePlayNpcInformations<'a> {
    pub base: GameRolePlayActorInformations<'a>,
    #[protocol(var)]
    pub npc_id: u16,
    pub sex: bool,
    #[protocol(var)]
    pub special_artwork_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 36)]
pub struct GameRolePlayCharacterInformations<'a> {
    pub base: GameRolePlayHumanoidInformations<'a>,
    pub alignment_infos: ActorAlignmentInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 410)]
pub struct HumanOptionFollowers<'a> {
    pub base: HumanOption<'a>,
    pub following_characters_look: std::borrow::Cow<'a, [IndexedEntityLook<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 140)]
pub struct GroupMonsterStaticInformations<'a> {
    pub main_creature_light_infos: MonsterInGroupLightInformations<'a>,
    pub underlings: std::borrow::Cow<'a, [MonsterInGroupInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 129)]
pub struct GameRolePlayMerchantInformations<'a> {
    pub base: GameRolePlayNamedActorInformations<'a>,
    pub sell_type: u8,
    pub options: std::borrow::Cow<'a, [HumanOptionVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 495)]
pub struct HumanOptionSkillUse<'a> {
    pub base: HumanOption<'a>,
    #[protocol(var)]
    pub element_id: u32,
    #[protocol(var)]
    pub skill_id: u16,
    pub skill_end_time: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 417)]
pub struct AllianceInformations<'a> {
    pub base: BasicNamedAllianceInformations<'a>,
    pub alliance_emblem: GuildEmblem<'a>,
}
