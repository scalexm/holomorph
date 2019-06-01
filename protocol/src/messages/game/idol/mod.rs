use crate::types::game::idol::PartyIdol;
use crate::variants::IdolVariant;
use crate::variants::PartyIdolVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6584)]
pub struct IdolSelectErrorMessage<'a> {
    #[protocol(flag)]
    pub activate: bool,
    #[protocol(flag)]
    pub party: bool,
    pub reason: u8,
    #[protocol(var)]
    pub idol_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6585)]
pub struct IdolListMessage<'a> {
    #[protocol(var_contents)]
    pub chosen_idols: std::borrow::Cow<'a, [u16]>,
    #[protocol(var_contents)]
    pub party_chosen_idols: std::borrow::Cow<'a, [u16]>,
    pub party_idols: std::borrow::Cow<'a, [PartyIdolVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6582)]
pub struct IdolPartyRegisterRequestMessage<'a> {
    pub register: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6583)]
pub struct IdolPartyRefreshMessage<'a> {
    pub party_idol: PartyIdol<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6586)]
pub struct IdolFightPreparationUpdateMessage<'a> {
    pub idol_source: u8,
    pub idols: std::borrow::Cow<'a, [IdolVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6587)]
pub struct IdolSelectRequestMessage<'a> {
    #[protocol(flag)]
    pub activate: bool,
    #[protocol(flag)]
    pub party: bool,
    #[protocol(var)]
    pub idol_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6581)]
pub struct IdolSelectedMessage<'a> {
    #[protocol(flag)]
    pub activate: bool,
    #[protocol(flag)]
    pub party: bool,
    #[protocol(var)]
    pub idol_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6580)]
pub struct IdolPartyLostMessage<'a> {
    #[protocol(var)]
    pub idol_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
