use crate::messages::game::social::BulletinMessage;
use crate::messages::game::social::SocialNoticeMessage;
use crate::messages::game::social::SocialNoticeSetErrorMessage;
use crate::messages::game::social::SocialNoticeSetRequestMessage;
use crate::types::game::context::roleplay::AllianceInformations;
use crate::types::game::context::roleplay::BasicAllianceInformations;
use crate::types::game::context::roleplay::BasicNamedAllianceInformations;
use crate::types::game::context::roleplay::GuildInAllianceInformations;
use crate::types::game::guild::GuildEmblem;
use crate::types::game::social::AllianceFactSheetInformations;
use crate::types::game::social::AllianceVersatileInformations;
use crate::types::game::social::GuildInsiderFactSheetInformations;
use crate::variants::AllianceFactSheetInformationsVariant;
use crate::variants::PrismSubareaEmptyInfoVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6403)]
pub struct AllianceInsiderInfoMessage<'a> {
    pub alliance_infos: AllianceFactSheetInformations<'a>,
    pub guilds: std::borrow::Cow<'a, [GuildInsiderFactSheetInformations<'a>]>,
    pub prisms: std::borrow::Cow<'a, [PrismSubareaEmptyInfoVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6395)]
pub struct AllianceInvitationMessage<'a> {
    #[protocol(var)]
    pub target_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6417)]
pub struct AllianceInsiderInfoRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6690)]
pub struct AllianceBulletinMessage<'a> {
    pub base: BulletinMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6402)]
pub struct AllianceJoinedMessage<'a> {
    pub alliance_info: AllianceInformations<'a>,
    pub enabled: bool,
    #[protocol(var)]
    pub leading_guild_id: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6427)]
pub struct AlliancePartialListMessage<'a> {
    pub base: AllianceListMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6400)]
pub struct AllianceKickRequestMessage<'a> {
    #[protocol(var)]
    pub kicked_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6423)]
pub struct AllianceFactsErrorMessage<'a> {
    #[protocol(var)]
    pub alliance_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6683)]
pub struct AllianceMotdSetErrorMessage<'a> {
    pub base: SocialNoticeSetErrorMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6398)]
pub struct AllianceLeftMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6447)]
pub struct AllianceModificationEmblemValidMessage<'a> {
    pub alliancemblem: GuildEmblem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6685)]
pub struct AllianceMotdMessage<'a> {
    pub base: SocialNoticeMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6692)]
pub struct AllianceBulletinSetErrorMessage<'a> {
    pub base: SocialNoticeSetErrorMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6391)]
pub struct AllianceCreationResultMessage<'a> {
    pub result: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6396)]
pub struct AllianceInvitationStateRecruterMessage<'a> {
    pub recruted_name: &'a str,
    pub invitation_state: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6394)]
pub struct AllianceCreationStartedMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6439)]
pub struct KohUpdateMessage<'a> {
    pub alliances: std::borrow::Cow<'a, [AllianceInformations<'a>]>,
    #[protocol(var_contents)]
    pub alliance_nb_members: std::borrow::Cow<'a, [u16]>,
    #[protocol(var_contents)]
    pub alliance_round_weigth: std::borrow::Cow<'a, [u32]>,
    pub alliance_match_score: &'a [u8],
    pub alliance_map_winners: std::borrow::Cow<'a, [BasicAllianceInformations<'a>]>,
    #[protocol(var)]
    pub alliance_map_winner_score: u32,
    #[protocol(var)]
    pub alliance_map_my_alliance_score: u32,
    pub next_tick_time: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6449)]
pub struct AllianceModificationNameAndTagValidMessage<'a> {
    pub alliance_name: &'a str,
    pub alliance_tag: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6401)]
pub struct AllianceInvitationAnswerMessage<'a> {
    pub accept: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6687)]
pub struct AllianceMotdSetRequestMessage<'a> {
    pub base: SocialNoticeSetRequestMessage<'a>,
    pub content: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6693)]
pub struct AllianceBulletinSetRequestMessage<'a> {
    pub base: SocialNoticeSetRequestMessage<'a>,
    pub content: &'a str,
    pub notify_members: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6426)]
pub struct AllianceChangeGuildRightsMessage<'a> {
    #[protocol(var)]
    pub guild_id: u32,
    pub rights: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6409)]
pub struct AllianceFactsRequestMessage<'a> {
    #[protocol(var)]
    pub alliance_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6392)]
pub struct AllianceInvitationStateRecrutedMessage<'a> {
    pub invitation_state: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6408)]
pub struct AllianceListMessage<'a> {
    pub alliances: std::borrow::Cow<'a, [AllianceFactSheetInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6414)]
pub struct AllianceFactsMessage<'a> {
    pub infos: AllianceFactSheetInformationsVariant<'a>,
    pub guilds: std::borrow::Cow<'a, [GuildInAllianceInformations<'a>]>,
    #[protocol(var_contents)]
    pub controlled_subarea_ids: std::borrow::Cow<'a, [u16]>,
    #[protocol(var)]
    pub leader_character_id: u64,
    pub leader_character_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6390)]
pub struct AllianceMembershipMessage<'a> {
    pub base: AllianceJoinedMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6399)]
pub struct AllianceGuildLeavingMessage<'a> {
    pub kicked: bool,
    #[protocol(var)]
    pub guild_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6444)]
pub struct AllianceModificationStartedMessage<'a> {
    #[protocol(flag)]
    pub can_change_name: bool,
    #[protocol(flag)]
    pub can_change_tag: bool,
    #[protocol(flag)]
    pub can_change_emblem: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6393)]
pub struct AllianceCreationValidMessage<'a> {
    pub alliance_name: &'a str,
    pub alliance_tag: &'a str,
    pub alliance_emblem: GuildEmblem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6397)]
pub struct AllianceInvitedMessage<'a> {
    #[protocol(var)]
    pub recruter_id: u64,
    pub recruter_name: &'a str,
    pub alliance_info: BasicNamedAllianceInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6450)]
pub struct AllianceModificationValidMessage<'a> {
    pub alliance_name: &'a str,
    pub alliance_tag: &'a str,
    pub alliancemblem: GuildEmblem<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6436)]
pub struct AllianceVersatileInfoListMessage<'a> {
    pub alliances: std::borrow::Cow<'a, [AllianceVersatileInformations<'a>]>,
}
