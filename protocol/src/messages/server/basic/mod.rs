use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 189)]
pub struct SystemMessageDisplayMessage<'a> {
    pub hang_up: bool,
    #[protocol(var)]
    pub msg_id: u16,
    pub parameters: std::borrow::Cow<'a, [&'a str]>,
}
