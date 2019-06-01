use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 547)]
pub struct RecycledItem<'a> {
    #[protocol(var)]
    pub id: u16,
    pub qty: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
