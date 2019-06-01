use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6726)]
pub struct HouseTeleportRequestMessage<'a> {
    #[protocol(var)]
    pub house_id: u32,
    pub house_instance_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
