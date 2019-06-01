use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6463)]
pub struct ClientUIOpenedByObjectMessage<'a> {
    pub base: ClientUIOpenedMessage<'a>,
    #[protocol(var)]
    pub uid: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6459)]
pub struct ClientUIOpenedMessage<'a> {
    pub type_: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
