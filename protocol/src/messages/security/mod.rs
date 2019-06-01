use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5607)]
pub struct ClientKeyMessage<'a> {
    pub key: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6372)]
pub struct CheckIntegrityMessage<'a> {
    #[protocol(var)]
    pub data: &'a [i8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6253)]
pub struct RawDataMessage<'a> {
    #[protocol(var)]
    pub content: &'a [u8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6156)]
pub struct CheckFileMessage<'a> {
    pub filename_hash: &'a str,
    pub type_: u8,
    pub value: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6154)]
pub struct CheckFileRequestMessage<'a> {
    pub filename: &'a str,
    pub type_: u8,
}
