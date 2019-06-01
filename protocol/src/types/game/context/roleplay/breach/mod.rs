use crate::types::game::context::roleplay::MonsterInGroupLightInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 559)]
pub struct BreachReward<'a> {
    #[protocol(var)]
    pub id: u32,
    pub buy_locks: &'a [u8],
    pub buy_criterion: &'a str,
    pub bought: bool,
    #[protocol(var)]
    pub price: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 558)]
pub struct BreachBranch<'a> {
    pub room: u8,
    pub element: u32,
    pub bosses: std::borrow::Cow<'a, [MonsterInGroupLightInformations<'a>]>,
    pub map: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 560)]
pub struct ExtendedBreachBranch<'a> {
    pub base: BreachBranch<'a>,
    pub monsters: std::borrow::Cow<'a, [MonsterInGroupLightInformations<'a>]>,
    pub rewards: std::borrow::Cow<'a, [BreachReward<'a>]>,
    #[protocol(var)]
    pub modifier: u32,
    #[protocol(var)]
    pub prize: u32,
}
