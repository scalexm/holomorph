pub mod entity;

use crate::types::game::character::choice::CharacterBaseInformations;
use crate::types::game::context::roleplay::party::entity::PartyEntityBaseInformation;
use crate::types::game::look::EntityLook;
use crate::variants::PartyEntityBaseInformationVariant;
use crate::variants::PlayerStatusVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 469)]
pub struct NamedPartyTeam<'a> {
    pub team_id: u8,
    pub party_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 470)]
pub struct NamedPartyTeamWithOutcome<'a> {
    pub team: NamedPartyTeam<'a>,
    #[protocol(var)]
    pub outcome: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 376)]
pub struct PartyInvitationMemberInformations<'a> {
    pub base: CharacterBaseInformations<'a>,
    pub world_x: i16,
    pub world_y: i16,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub entities: std::borrow::Cow<'a, [PartyEntityBaseInformation<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 378)]
pub struct PartyMemberGeoPosition<'a> {
    pub member_id: u32,
    pub world_x: i16,
    pub world_y: i16,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 90)]
pub struct PartyMemberInformations<'a> {
    pub base: CharacterBaseInformations<'a>,
    #[protocol(var)]
    pub life_points: u32,
    #[protocol(var)]
    pub max_life_points: u32,
    #[protocol(var)]
    pub prospecting: u16,
    pub regen_rate: u8,
    #[protocol(var)]
    pub initiative: u16,
    pub alignment_side: i8,
    pub world_x: i16,
    pub world_y: i16,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
    pub status: PlayerStatusVariant<'a>,
    pub entities: std::borrow::Cow<'a, [PartyEntityBaseInformationVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 391)]
pub struct PartyMemberArenaInformations<'a> {
    pub base: PartyMemberInformations<'a>,
    #[protocol(var)]
    pub rank: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 374)]
pub struct PartyGuestInformations<'a> {
    #[protocol(var)]
    pub guest_id: u64,
    #[protocol(var)]
    pub host_id: u64,
    pub name: &'a str,
    pub guest_look: EntityLook<'a>,
    pub breed: i8,
    pub sex: bool,
    pub status: PlayerStatusVariant<'a>,
    pub entities: std::borrow::Cow<'a, [PartyEntityBaseInformation<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 373)]
pub struct DungeonPartyFinderPlayer<'a> {
    #[protocol(var)]
    pub player_id: u64,
    pub player_name: &'a str,
    pub breed: i8,
    pub sex: bool,
    #[protocol(var)]
    pub level: u16,
}
