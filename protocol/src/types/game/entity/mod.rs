use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 546)]
pub struct EntityInformation<'a> {
    #[protocol(var)]
    pub id: u16,
    #[protocol(var)]
    pub experience: u32,
    pub status: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
