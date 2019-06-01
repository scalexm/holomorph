pub mod branch;
pub mod meeting;
pub mod reward;

use crate::messages::game::context::roleplay::MapComplementaryInformationsDataMessage;
use crate::types::game::character::CharacterMinimalInformations;
use crate::types::game::context::roleplay::breach::BreachBranch;
use crate::types::game::data::items::effects::ObjectEffectInteger;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6810)]
pub struct BreachEnterMessage<'a> {
    #[protocol(var)]
    pub owner: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6786)]
pub struct BreachBudgetMessage<'a> {
    #[protocol(var)]
    pub bugdet: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6815)]
pub struct BreachExitRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6811)]
pub struct BreachCharactersMessage<'a> {
    #[protocol(var_contents)]
    pub characters: std::borrow::Cow<'a, [u64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6799)]
pub struct BreachStateMessage<'a> {
    pub owner: CharacterMinimalInformations<'a>,
    pub bonuses: std::borrow::Cow<'a, [ObjectEffectInteger<'a>]>,
    #[protocol(var)]
    pub bugdet: u32,
    pub saved: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6791)]
pub struct MapComplementaryInformationsBreachMessage<'a> {
    pub base: MapComplementaryInformationsDataMessage<'a>,
    #[protocol(var)]
    pub floor: u32,
    pub room: u8,
    pub branches: std::borrow::Cow<'a, [BreachBranch<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6817)]
pub struct BreachTeleportRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6800)]
pub struct BreachBonusMessage<'a> {
    pub bonus: ObjectEffectInteger<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6798)]
pub struct BreachSavedMessage<'a> {
    pub saved: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6814)]
pub struct BreachExitResponseMessage<'a> {
    pub exited: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6816)]
pub struct BreachTeleportResponseMessage<'a> {
    pub teleported: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
