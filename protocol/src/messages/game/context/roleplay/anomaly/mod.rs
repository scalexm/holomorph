use crate::messages::game::context::roleplay::MapComplementaryInformationsDataMessage;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6828)]
pub struct MapComplementaryInformationsAnomalyMessage<'a> {
    pub base: MapComplementaryInformationsDataMessage<'a>,
    #[protocol(var)]
    pub level: u16,
    #[protocol(var)]
    pub closing_time: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6831)]
pub struct AnomalyStateMessage<'a> {
    #[protocol(var)]
    pub sub_area_id: u16,
    pub open: bool,
    #[protocol(var)]
    pub closing_time: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
