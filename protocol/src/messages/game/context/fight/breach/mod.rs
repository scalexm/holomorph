use crate::messages::game::context::fight::GameFightEndMessage;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6809)]
pub struct BreachGameFightEndMessage<'a> {
    pub base: GameFightEndMessage<'a>,
    pub budget: i32,
}
