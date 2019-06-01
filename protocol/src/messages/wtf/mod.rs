use crate::messages::debug::DebugInClientMessage;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6594)]
pub struct ClientYouAreDrunkMessage<'a> {
    pub base: DebugInClientMessage<'a>,
}
