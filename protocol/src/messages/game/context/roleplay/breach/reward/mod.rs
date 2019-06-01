use crate::types::game::context::roleplay::breach::BreachReward;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6803)]
pub struct BreachRewardBuyMessage<'a> {
    #[protocol(var)]
    pub id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6787)]
pub struct BreachSaveBuyMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6813)]
pub struct BreachRewardsMessage<'a> {
    pub rewards: std::borrow::Cow<'a, [BreachReward<'a>]>,
    pub save: BreachReward<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6788)]
pub struct BreachSaveBoughtMessage<'a> {
    pub bought: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6797)]
pub struct BreachRewardBoughtMessage<'a> {
    #[protocol(var)]
    pub id: u32,
    pub bought: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
