use crate::types::game::interactive::zaap::TeleportDestination;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6830)]
pub struct ZaapDestinationsMessage<'a> {
    pub base: TeleportDestinationsMessage<'a>,
    pub spawn_map_id: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6572)]
pub struct ZaapRespawnSaveRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6571)]
pub struct ZaapRespawnUpdatedMessage<'a> {
    pub map_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6829)]
pub struct TeleportDestinationsMessage<'a> {
    pub type_: u8,
    pub destinations: std::borrow::Cow<'a, [TeleportDestination<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5961)]
pub struct TeleportRequestMessage<'a> {
    pub source_type: u8,
    pub destination_type: u8,
    pub map_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
