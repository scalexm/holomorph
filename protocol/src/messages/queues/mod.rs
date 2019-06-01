use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6100)]
pub struct QueueStatusMessage<'a> {
    pub position: u16,
    pub total: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 10)]
pub struct LoginQueueStatusMessage<'a> {
    pub position: u16,
    pub total: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
