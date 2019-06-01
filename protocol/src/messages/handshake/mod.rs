use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1)]
pub struct ProtocolRequired<'a> {
    pub required_version: u32,
    pub current_version: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
