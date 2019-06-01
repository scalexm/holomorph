use crate::variants::MapCoordinatesVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5584)]
pub struct CompassResetMessage<'a> {
    pub type_: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5591)]
pub struct CompassUpdateMessage<'a> {
    pub type_: u8,
    pub coords: MapCoordinatesVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6013)]
pub struct CompassUpdatePvpSeekMessage<'a> {
    pub base: CompassUpdateMessage<'a>,
    #[protocol(var)]
    pub member_id: u64,
    pub member_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5589)]
pub struct CompassUpdatePartyMemberMessage<'a> {
    pub base: CompassUpdateMessage<'a>,
    #[protocol(var)]
    pub member_id: u64,
    pub active: bool,
}
