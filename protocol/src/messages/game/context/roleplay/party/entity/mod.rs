use crate::messages::game::context::roleplay::party::PartyUpdateLightMessage;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6781)]
pub struct PartyEntityUpdateLightMessage<'a> {
    pub base: PartyUpdateLightMessage<'a>,
    pub index_id: u8,
}
