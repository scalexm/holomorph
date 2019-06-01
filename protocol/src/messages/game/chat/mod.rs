pub mod channel;
pub mod community;
pub mod smiley;

use crate::types::game::data::items::ObjectItem;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 870)]
pub struct ChatErrorMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6135)]
pub struct ChatAdminServerMessage<'a> {
    pub base: ChatServerMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 862)]
pub struct ChatClientMultiWithObjectMessage<'a> {
    pub base: ChatClientMultiMessage<'a>,
    pub objects: std::borrow::Cow<'a, [ObjectItem<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 882)]
pub struct ChatServerCopyMessage<'a> {
    pub base: ChatAbstractServerMessage<'a>,
    #[protocol(var)]
    pub receiver_id: u64,
    pub receiver_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 881)]
pub struct ChatServerMessage<'a> {
    pub base: ChatAbstractServerMessage<'a>,
    pub sender_id: f64,
    pub sender_name: &'a str,
    pub prefix: &'a str,
    pub sender_account_id: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 850)]
pub struct ChatAbstractClientMessage<'a> {
    pub content: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 884)]
pub struct ChatServerCopyWithObjectMessage<'a> {
    pub base: ChatServerCopyMessage<'a>,
    pub objects: std::borrow::Cow<'a, [ObjectItem<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 883)]
pub struct ChatServerWithObjectMessage<'a> {
    pub base: ChatServerMessage<'a>,
    pub objects: std::borrow::Cow<'a, [ObjectItem<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 851)]
pub struct ChatClientPrivateMessage<'a> {
    pub base: ChatAbstractClientMessage<'a>,
    pub receiver: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 880)]
pub struct ChatAbstractServerMessage<'a> {
    pub channel: u8,
    pub content: &'a str,
    pub timestamp: u32,
    pub fingerprint: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 852)]
pub struct ChatClientPrivateWithObjectMessage<'a> {
    pub base: ChatClientPrivateMessage<'a>,
    pub objects: std::borrow::Cow<'a, [ObjectItem<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 861)]
pub struct ChatClientMultiMessage<'a> {
    pub base: ChatAbstractClientMessage<'a>,
    pub channel: u8,
}
