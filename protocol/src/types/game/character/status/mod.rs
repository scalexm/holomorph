use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 414)]
pub struct PlayerStatusExtended<'a> {
    pub base: PlayerStatus<'a>,
    pub message: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 415)]
pub struct PlayerStatus<'a> {
    pub status_id: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
