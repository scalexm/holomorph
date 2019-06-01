use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6299)]
pub struct DungeonKeyRingMessage<'a> {
    #[protocol(var_contents)]
    pub availables: std::borrow::Cow<'a, [u16]>,
    #[protocol(var_contents)]
    pub unavailables: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6296)]
pub struct DungeonKeyRingUpdateMessage<'a> {
    #[protocol(var)]
    pub dungeon_id: u16,
    pub available: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
