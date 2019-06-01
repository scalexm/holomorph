use crate::types::game::character::CharacterBasicMinimalInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 501)]
pub struct DareCriteria<'a> {
    pub type_: u8,
    pub params: std::borrow::Cow<'a, [i32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 504)]
pub struct DareVersatileInformations<'a> {
    pub dare_id: f64,
    pub count_entrants: u32,
    pub count_winners: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 502)]
pub struct DareInformations<'a> {
    pub dare_id: f64,
    pub creator: CharacterBasicMinimalInformations<'a>,
    #[protocol(var)]
    pub subscription_fee: u64,
    #[protocol(var)]
    pub jackpot: u64,
    pub max_count_winners: u16,
    pub end_date: f64,
    pub is_private: bool,
    #[protocol(var)]
    pub guild_id: u32,
    #[protocol(var)]
    pub alliance_id: u32,
    pub criterions: std::borrow::Cow<'a, [DareCriteria<'a>]>,
    pub start_date: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 505)]
pub struct DareReward<'a> {
    pub type_: u8,
    #[protocol(var)]
    pub monster_id: u16,
    #[protocol(var)]
    pub kamas: u64,
    pub dare_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
