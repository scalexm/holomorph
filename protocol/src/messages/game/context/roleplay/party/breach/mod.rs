use crate::messages::game::context::roleplay::party::AbstractPartyMemberInFightMessage;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6824)]
pub struct PartyMemberInBreachFightMessage<'a> {
    pub base: AbstractPartyMemberInFightMessage<'a>,
    #[protocol(var)]
    pub floor: u32,
    pub room: u8,
}
