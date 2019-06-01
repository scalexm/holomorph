use crate::messages::game::interactive::InteractiveUseRequestMessage;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6715)]
pub struct InteractiveUseWithParamRequestMessage<'a> {
    pub base: InteractiveUseRequestMessage<'a>,
    pub id: i32,
}
