use crate::types::game::character::CharacterMinimalInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6795)]
pub struct BreachInvitationAnswerMessage<'a> {
    pub accept: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6805)]
pub struct BreachInvitationOfferMessage<'a> {
    pub host: CharacterMinimalInformations<'a>,
    #[protocol(var)]
    pub time_left_before_cancel: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6789)]
pub struct BreachKickResponseMessage<'a> {
    pub target: CharacterMinimalInformations<'a>,
    pub kicked: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6794)]
pub struct BreachInvitationRequestMessage<'a> {
    #[protocol(var)]
    pub guest: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6790)]
pub struct BreachInvitationCloseMessage<'a> {
    pub host: CharacterMinimalInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6804)]
pub struct BreachKickRequestMessage<'a> {
    #[protocol(var)]
    pub target: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6792)]
pub struct BreachInvitationResponseMessage<'a> {
    pub guest: CharacterMinimalInformations<'a>,
    pub accept: bool,
}
