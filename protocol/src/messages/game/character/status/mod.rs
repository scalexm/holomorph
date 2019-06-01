use crate::variants::PlayerStatusVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6385)]
pub struct PlayerStatusUpdateErrorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6387)]
pub struct PlayerStatusUpdateRequestMessage<'a> {
    pub status: PlayerStatusVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6386)]
pub struct PlayerStatusUpdateMessage<'a> {
    pub account_id: u32,
    #[protocol(var)]
    pub player_id: u64,
    pub status: PlayerStatusVariant<'a>,
}
