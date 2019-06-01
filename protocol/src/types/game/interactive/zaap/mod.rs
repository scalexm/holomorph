use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 563)]
pub struct TeleportDestination<'a> {
    pub type_: u8,
    pub map_id: f64,
    #[protocol(var)]
    pub sub_area_id: u16,
    #[protocol(var)]
    pub level: u16,
    #[protocol(var)]
    pub cost: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
