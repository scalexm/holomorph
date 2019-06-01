use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6127)]
pub struct ConsoleCommandsListMessage<'a> {
    pub aliases: std::borrow::Cow<'a, [&'a str]>,
    pub args: std::borrow::Cow<'a, [&'a str]>,
    pub descriptions: std::borrow::Cow<'a, [&'a str]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5662)]
pub struct AdminQuietCommandMessage<'a> {
    pub base: AdminCommandMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 76)]
pub struct AdminCommandMessage<'a> {
    pub content: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 75)]
pub struct ConsoleMessage<'a> {
    pub type_: u8,
    pub content: &'a str,
}
