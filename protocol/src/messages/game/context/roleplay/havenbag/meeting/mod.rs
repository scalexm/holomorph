use crate::types::game::character::CharacterMinimalInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6713)]
pub struct HavenBagPermissionsUpdateMessage<'a> {
    pub permissions: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6642)]
pub struct InviteInHavenBagMessage<'a> {
    pub guest_informations: CharacterMinimalInformations<'a>,
    pub accept: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6643)]
pub struct InviteInHavenBagOfferMessage<'a> {
    pub host_informations: CharacterMinimalInformations<'a>,
    #[protocol(var)]
    pub time_left_before_cancel: i32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6647)]
pub struct TeleportHavenBagRequestMessage<'a> {
    #[protocol(var)]
    pub guest_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6646)]
pub struct TeleportHavenBagAnswerMessage<'a> {
    pub accept: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6714)]
pub struct HavenBagPermissionsUpdateRequestMessage<'a> {
    pub permissions: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6645)]
pub struct InviteInHavenBagClosedMessage<'a> {
    pub host_informations: CharacterMinimalInformations<'a>,
}
