use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6134)]
pub struct PopupWarningMessage<'a> {
    pub lock_duration: u8,
    pub author: &'a str,
    pub content: &'a str,
}
