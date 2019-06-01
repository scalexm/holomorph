pub mod breach;
pub mod entity;

use crate::types::game::context::roleplay::party::DungeonPartyFinderPlayer;
use crate::types::game::context::roleplay::party::PartyGuestInformations;
use crate::types::game::context::roleplay::party::PartyMemberGeoPosition;
use crate::types::game::context::MapCoordinatesExtended;
use crate::variants::PartyInvitationMemberInformationsVariant;
use crate::variants::PartyMemberInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5575)]
pub struct PartyUpdateMessage<'a> {
    pub base: AbstractPartyEventMessage<'a>,
    pub member_informations: PartyMemberInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5586)]
pub struct PartyInvitationMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub party_type: u8,
    pub party_name: &'a str,
    pub max_participants: u8,
    #[protocol(var)]
    pub from_id: u64,
    pub from_name: &'a str,
    #[protocol(var)]
    pub to_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5590)]
pub struct PartyKickedByMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    #[protocol(var)]
    pub kicker_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6241)]
pub struct DungeonPartyFinderRegisterSuccessMessage<'a> {
    #[protocol(var_contents)]
    pub dungeon_ids: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6251)]
pub struct PartyCancelInvitationNotificationMessage<'a> {
    pub base: AbstractPartyEventMessage<'a>,
    #[protocol(var)]
    pub canceler_id: u64,
    #[protocol(var)]
    pub guest_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6283)]
pub struct PartyInvitationArenaRequestMessage<'a> {
    pub base: PartyInvitationRequestMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6246)]
pub struct DungeonPartyFinderListenRequestMessage<'a> {
    #[protocol(var)]
    pub dungeon_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5585)]
pub struct PartyInvitationRequestMessage<'a> {
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5583)]
pub struct PartyCannotJoinErrorMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub reason: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6249)]
pub struct DungeonPartyFinderRegisterRequestMessage<'a> {
    #[protocol(var_contents)]
    pub dungeon_ids: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5592)]
pub struct PartyKickRequestMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    #[protocol(var)]
    pub player_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5595)]
pub struct PartyLocateMembersMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub geopositions: std::borrow::Cow<'a, [PartyMemberGeoPosition<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6273)]
pub struct AbstractPartyEventMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5587)]
pub struct PartyLocateMembersRequestMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6256)]
pub struct PartyInvitationCancelledForGuestMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    #[protocol(var)]
    pub canceler_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6245)]
pub struct PartyInvitationDungeonRequestMessage<'a> {
    pub base: PartyInvitationRequestMessage<'a>,
    #[protocol(var)]
    pub dungeon_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5594)]
pub struct PartyLeaveMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6825)]
pub struct AbstractPartyMemberInFightMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub reason: u8,
    #[protocol(var)]
    pub member_id: u64,
    pub member_account_id: u32,
    pub member_name: &'a str,
    #[protocol(var)]
    pub fight_id: u16,
    #[protocol(var)]
    pub time_before_fight_start: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6826)]
pub struct PartyMemberInStandardFightMessage<'a> {
    pub base: AbstractPartyMemberInFightMessage<'a>,
    pub fight_map: MapCoordinatesExtended<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6240)]
pub struct DungeonPartyFinderAvailableDungeonsRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6306)]
pub struct PartyNewMemberMessage<'a> {
    pub base: PartyUpdateMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5579)]
pub struct PartyMemberRemoveMessage<'a> {
    pub base: AbstractPartyEventMessage<'a>,
    #[protocol(var)]
    pub leaving_player_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6243)]
pub struct DungeonPartyFinderRegisterErrorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6270)]
pub struct PartyLoyaltyStatusMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub loyal: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5580)]
pub struct PartyAcceptInvitationMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6502)]
pub struct PartyNameUpdateMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub party_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6264)]
pub struct PartyInvitationDetailsRequestMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5574)]
pub struct PartyStopFollowRequestMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    #[protocol(var)]
    pub player_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6247)]
pub struct DungeonPartyFinderRoomContentMessage<'a> {
    #[protocol(var)]
    pub dungeon_id: u16,
    pub players: std::borrow::Cow<'a, [DungeonPartyFinderPlayer<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6054)]
pub struct PartyUpdateLightMessage<'a> {
    pub base: AbstractPartyEventMessage<'a>,
    #[protocol(var)]
    pub id: u64,
    #[protocol(var)]
    pub life_points: u32,
    #[protocol(var)]
    pub max_life_points: u32,
    #[protocol(var)]
    pub prospecting: u16,
    pub regen_rate: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5596)]
pub struct PartyRefuseInvitationNotificationMessage<'a> {
    pub base: AbstractPartyEventMessage<'a>,
    #[protocol(var)]
    pub guest_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6501)]
pub struct PartyNameSetErrorMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub result: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6262)]
pub struct PartyInvitationDungeonDetailsMessage<'a> {
    pub base: PartyInvitationDetailsMessage<'a>,
    #[protocol(var)]
    pub dungeon_id: u16,
    pub players_dungeon_ready: std::borrow::Cow<'a, [bool]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5578)]
pub struct PartyLeaderUpdateMessage<'a> {
    pub base: AbstractPartyEventMessage<'a>,
    #[protocol(var)]
    pub party_leader_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6250)]
pub struct DungeonPartyFinderRoomContentUpdateMessage<'a> {
    #[protocol(var)]
    pub dungeon_id: u16,
    pub added_players: std::borrow::Cow<'a, [DungeonPartyFinderPlayer<'a>]>,
    #[protocol(var_contents)]
    pub removed_players_ids: std::borrow::Cow<'a, [u64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6248)]
pub struct DungeonPartyFinderListenErrorMessage<'a> {
    #[protocol(var)]
    pub dungeon_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5577)]
pub struct PartyFollowMemberRequestMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    #[protocol(var)]
    pub player_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5582)]
pub struct PartyRefuseInvitationMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6263)]
pub struct PartyInvitationDetailsMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub party_type: u8,
    pub party_name: &'a str,
    #[protocol(var)]
    pub from_id: u64,
    pub from_name: &'a str,
    #[protocol(var)]
    pub leader_id: u64,
    pub members: std::borrow::Cow<'a, [PartyInvitationMemberInformationsVariant<'a>]>,
    pub guests: std::borrow::Cow<'a, [PartyGuestInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5588)]
pub struct PartyFollowThisMemberRequestMessage<'a> {
    pub base: PartyFollowMemberRequestMessage<'a>,
    pub enabled: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6254)]
pub struct PartyCancelInvitationMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    #[protocol(var)]
    pub guest_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6175)]
pub struct PartyRestrictedMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub restricted: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6274)]
pub struct AbstractPartyMessage<'a> {
    #[protocol(var)]
    pub party_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5581)]
pub struct PartyFollowStatusUpdateMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    #[protocol(flag)]
    pub success: bool,
    #[protocol(flag)]
    pub is_followed: bool,
    #[protocol(var)]
    pub followed_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6260)]
pub struct PartyNewGuestMessage<'a> {
    pub base: AbstractPartyEventMessage<'a>,
    pub guest: PartyGuestInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6242)]
pub struct DungeonPartyFinderAvailableDungeonsMessage<'a> {
    #[protocol(var_contents)]
    pub dungeon_ids: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5576)]
pub struct PartyJoinMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub party_type: u8,
    #[protocol(var)]
    pub party_leader_id: u64,
    pub max_participants: u8,
    pub members: std::borrow::Cow<'a, [PartyMemberInformationsVariant<'a>]>,
    pub guests: std::borrow::Cow<'a, [PartyGuestInformations<'a>]>,
    pub restricted: bool,
    pub party_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6080)]
pub struct PartyAbdicateThroneMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    #[protocol(var)]
    pub player_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6252)]
pub struct PartyMemberEjectedMessage<'a> {
    pub base: PartyMemberRemoveMessage<'a>,
    #[protocol(var)]
    pub kicker_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6261)]
pub struct PartyDeletedMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6277)]
pub struct PartyModifiableStatusMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub enabled: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5593)]
pub struct PartyLeaveRequestMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6244)]
pub struct PartyInvitationDungeonMessage<'a> {
    pub base: PartyInvitationMessage<'a>,
    #[protocol(var)]
    pub dungeon_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6269)]
pub struct PartyPledgeLoyaltyRequestMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub loyal: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6503)]
pub struct PartyNameSetRequestMessage<'a> {
    pub base: AbstractPartyMessage<'a>,
    pub party_name: &'a str,
}
