pub mod arena;

use crate::types::game::context::fight::FightCommonInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6073)]
pub struct GameRolePlayAggressionMessage<'a> {
    #[protocol(var)]
    pub attacker_id: u64,
    #[protocol(var)]
    pub defender_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6741)]
pub struct GameRolePlayMonsterAngryAtPlayerMessage<'a> {
    #[protocol(var)]
    pub player_id: u64,
    pub monster_group_id: f64,
    pub angry_start_time: f64,
    pub attack_time: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5733)]
pub struct GameRolePlayPlayerFightFriendlyAnsweredMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    #[protocol(var)]
    pub source_id: u64,
    #[protocol(var)]
    pub target_id: u64,
    pub accept: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5937)]
pub struct GameRolePlayPlayerFightFriendlyRequestedMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    #[protocol(var)]
    pub source_id: u64,
    #[protocol(var)]
    pub target_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5731)]
pub struct GameRolePlayPlayerFightRequestMessage<'a> {
    #[protocol(var)]
    pub target_id: u64,
    pub target_cell_id: i16,
    pub friendly: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5732)]
pub struct GameRolePlayPlayerFightFriendlyAnswerMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub accept: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 300)]
pub struct GameRolePlayRemoveChallengeMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5822)]
pub struct GameRolePlayFightRequestCanceledMessage<'a> {
    #[protocol(var)]
    pub fight_id: u16,
    pub source_id: f64,
    pub target_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 301)]
pub struct GameRolePlayShowChallengeMessage<'a> {
    pub commons_infos: FightCommonInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6191)]
pub struct GameRolePlayAttackMonsterRequestMessage<'a> {
    pub monster_group_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6742)]
pub struct GameRolePlayMonsterNotAngryAtPlayerMessage<'a> {
    #[protocol(var)]
    pub player_id: u64,
    pub monster_group_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
