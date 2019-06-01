use crate::types::game::context::roleplay::fight::arena::ArenaRankInfos;
use crate::types::game::context::roleplay::fight::arena::LeagueFriendInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6282)]
pub struct GameRolePlayArenaUnregisterMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6785)]
pub struct GameRolePlayArenaLeagueRewardsMessage<'a> {
    #[protocol(var)]
    pub season_id: u16,
    #[protocol(var)]
    pub league_id: u16,
    pub ladder_position: i32,
    pub end_season_reward: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6574)]
pub struct GameRolePlayArenaSwitchToGameServerMessage<'a> {
    pub valid_token: bool,
    #[protocol(var)]
    pub ticket: &'a [i8],
    pub home_server_id: i16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6280)]
pub struct GameRolePlayArenaRegisterMessage<'a> {
    pub battle_mode: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6276)]
pub struct GameRolePlayArenaFightPropositionMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub allies_id: std::borrow::Cow<'a, [f64]>,
    #[protocol(var)]
    pub duration: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6575)]
pub struct GameRolePlayArenaSwitchToFightServerMessage<'a> {
    pub address: &'a str,
    pub ports: std::borrow::Cow<'a, [u16]>,
    #[protocol(var)]
    pub ticket: &'a [i8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6728)]
pub struct GameRolePlayArenaUpdatePlayerInfosAllQueuesMessage<'a> {
    pub base: GameRolePlayArenaUpdatePlayerInfosMessage<'a>,
    pub team: ArenaRankInfos<'a>,
    pub duel: ArenaRankInfos<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6284)]
pub struct GameRolePlayArenaRegistrationStatusMessage<'a> {
    pub registered: bool,
    pub step: u8,
    pub battle_mode: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6281)]
pub struct GameRolePlayArenaFighterStatusMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub player_id: f64,
    pub accepted: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6279)]
pub struct GameRolePlayArenaFightAnswerMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub accept: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6301)]
pub struct GameRolePlayArenaUpdatePlayerInfosMessage<'a> {
    pub solo: ArenaRankInfos<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6783)]
pub struct GameRolePlayArenaInvitationCandidatesAnswer<'a> {
    pub candidates: std::borrow::Cow<'a, [LeagueFriendInformations<'a>]>,
}
