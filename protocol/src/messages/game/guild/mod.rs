pub mod tax;

use crate::messages::game::social::BulletinMessage;
use crate::messages::game::social::SocialNoticeMessage;
use crate::messages::game::social::SocialNoticeSetErrorMessage;
use crate::messages::game::social::SocialNoticeSetRequestMessage;
use crate::types::game::character::CharacterMinimalGuildPublicInformations;
use crate::types::game::context::roleplay::BasicGuildInformations;
use crate::types::game::context::roleplay::BasicNamedAllianceInformations;
use crate::types::game::context::roleplay::GuildInformations;
use crate::types::game::guild::GuildEmblem;
use crate::types::game::guild::GuildMember;
use crate::types::game::house::HouseInformationsForGuild;
use crate::types::game::paddock::PaddockContentInformations;
use crate::variants::GuildFactSheetInformationsVariant;
use crate::variants::GuildVersatileInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6415)]
pub struct GuildFactsMessage<'a> {
    pub infos: GuildFactSheetInformationsVariant<'a>,
    pub creation_date: u32,
    #[protocol(var)]
    pub nb_tax_collectors: u16,
    pub members: std::borrow::Cow<'a, [CharacterMinimalGuildPublicInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5548)]
pub struct GuildInvitationStateRecrutedMessage<'a> {
    pub invitation_state: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6413)]
pub struct GuildListMessage<'a> {
    pub guilds: std::borrow::Cow<'a, [GuildInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5955)]
pub struct GuildPaddockRemovedMessage<'a> {
    pub paddock_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5564)]
pub struct GuildJoinedMessage<'a> {
    pub guild_info: GuildInformations<'a>,
    #[protocol(var)]
    pub member_rights: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6691)]
pub struct GuildBulletinSetErrorMessage<'a> {
    pub base: SocialNoticeSetErrorMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5908)]
pub struct ChallengeFightJoinRefusedMessage<'a> {
    #[protocol(var)]
    pub player_id: u64,
    pub reason: i8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5957)]
pub struct GuildPaddockTeleportRequestMessage<'a> {
    pub paddock_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5549)]
pub struct GuildChangeMemberParametersMessage<'a> {
    #[protocol(var)]
    pub member_id: u64,
    #[protocol(var)]
    pub rank: u16,
    pub experience_given_percent: u8,
    #[protocol(var)]
    pub rights: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5636)]
pub struct GuildInfosUpgradeMessage<'a> {
    pub max_tax_collectors_count: u8,
    pub tax_collectors_count: u8,
    #[protocol(var)]
    pub tax_collector_life_points: u16,
    #[protocol(var)]
    pub tax_collector_damages_bonuses: u16,
    #[protocol(var)]
    pub tax_collector_pods: u16,
    #[protocol(var)]
    pub tax_collector_prospecting: u16,
    #[protocol(var)]
    pub tax_collector_wisdom: u16,
    #[protocol(var)]
    pub boost_points: u16,
    #[protocol(var_contents)]
    pub spell_id: std::borrow::Cow<'a, [u16]>,
    pub spell_level: std::borrow::Cow<'a, [i16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5554)]
pub struct GuildCreationResultMessage<'a> {
    pub result: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6115)]
pub struct GuildInvitationByNameMessage<'a> {
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6181)]
pub struct GuildHouseUpdateInformationMessage<'a> {
    pub houses_informations: HouseInformationsForGuild<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5919)]
pub struct GuildHousesInformationMessage<'a> {
    pub houses_informations: std::borrow::Cow<'a, [HouseInformationsForGuild<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6689)]
pub struct GuildBulletinMessage<'a> {
    pub base: BulletinMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5552)]
pub struct GuildInvitedMessage<'a> {
    #[protocol(var)]
    pub recruter_id: u64,
    pub recruter_name: &'a str,
    pub guild_info: BasicGuildInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5959)]
pub struct GuildInformationsPaddocksMessage<'a> {
    pub nb_paddock_max: u8,
    pub paddocks_informations: std::borrow::Cow<'a, [PaddockContentInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6324)]
pub struct GuildModificationStartedMessage<'a> {
    #[protocol(flag)]
    pub can_change_name: bool,
    #[protocol(flag)]
    pub can_change_emblem: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5706)]
pub struct GuildCharacsUpgradeRequestMessage<'a> {
    pub chara_type_target: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5558)]
pub struct GuildInformationsMembersMessage<'a> {
    pub members: std::borrow::Cow<'a, [GuildMember<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6404)]
pub struct GuildFactsRequestMessage<'a> {
    #[protocol(var)]
    pub guild_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5835)]
pub struct GuildMembershipMessage<'a> {
    pub base: GuildJoinedMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5557)]
pub struct GuildInformationsGeneralMessage<'a> {
    pub abandonned_paddock: bool,
    pub level: u8,
    #[protocol(var)]
    pub exp_level_floor: u64,
    #[protocol(var)]
    pub experience: u64,
    #[protocol(var)]
    pub exp_next_level_floor: u64,
    pub creation_date: u32,
    #[protocol(var)]
    pub nb_total_members: u16,
    #[protocol(var)]
    pub nb_connected_members: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5887)]
pub struct GuildKickRequestMessage<'a> {
    #[protocol(var)]
    pub kicked_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5952)]
pub struct GuildPaddockBoughtMessage<'a> {
    pub paddock_info: PaddockContentInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6591)]
pub struct GuildMotdSetErrorMessage<'a> {
    pub base: SocialNoticeSetErrorMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5546)]
pub struct GuildCreationValidMessage<'a> {
    pub guild_name: &'a str,
    pub guild_emblem: GuildEmblem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6061)]
pub struct GuildMemberOnlineStatusMessage<'a> {
    #[protocol(var)]
    pub member_id: u64,
    pub online: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6327)]
pub struct GuildModificationNameValidMessage<'a> {
    pub guild_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6180)]
pub struct GuildHouseRemoveMessage<'a> {
    #[protocol(var)]
    pub house_id: u32,
    pub instance_id: u32,
    pub second_hand: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6694)]
pub struct GuildBulletinSetRequestMessage<'a> {
    pub base: SocialNoticeSetRequestMessage<'a>,
    pub content: &'a str,
    pub notify_members: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5597)]
pub struct GuildInformationsMemberUpdateMessage<'a> {
    pub member: GuildMember<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6422)]
pub struct GuildInAllianceFactsMessage<'a> {
    pub base: GuildFactsMessage<'a>,
    pub alliance_infos: BasicNamedAllianceInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5699)]
pub struct GuildSpellUpgradeRequestMessage<'a> {
    pub spell_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5563)]
pub struct GuildInvitationStateRecruterMessage<'a> {
    pub recruted_name: &'a str,
    pub invitation_state: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5551)]
pub struct GuildInvitationMessage<'a> {
    #[protocol(var)]
    pub target_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5923)]
pub struct GuildMemberLeavingMessage<'a> {
    pub kicked: bool,
    #[protocol(var)]
    pub member_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6588)]
pub struct GuildMotdSetRequestMessage<'a> {
    pub base: SocialNoticeSetRequestMessage<'a>,
    pub content: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5920)]
pub struct GuildCreationStartedMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5550)]
pub struct GuildGetInformationsMessage<'a> {
    pub info_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6590)]
pub struct GuildMotdMessage<'a> {
    pub base: SocialNoticeMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6328)]
pub struct GuildModificationEmblemValidMessage<'a> {
    pub guild_emblem: GuildEmblem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6435)]
pub struct GuildVersatileInfoListMessage<'a> {
    pub guilds: std::borrow::Cow<'a, [GuildVersatileInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6062)]
pub struct GuildLevelUpMessage<'a> {
    pub new_level: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5562)]
pub struct GuildLeftMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6424)]
pub struct GuildFactsErrorMessage<'a> {
    #[protocol(var)]
    pub guild_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6323)]
pub struct GuildModificationValidMessage<'a> {
    pub guild_name: &'a str,
    pub guild_emblem: GuildEmblem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5556)]
pub struct GuildInvitationAnswerMessage<'a> {
    pub accept: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
