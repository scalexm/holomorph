use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3017)]
pub struct ObjectGroundAddedMessage<'a> {
    #[protocol(var)]
    pub cell_id: u16,
    #[protocol(var)]
    pub object_gid: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5925)]
pub struct ObjectGroundListAddedMessage<'a> {
    #[protocol(var_contents)]
    pub cells: std::borrow::Cow<'a, [u16]>,
    #[protocol(var_contents)]
    pub reference_ids: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3014)]
pub struct ObjectGroundRemovedMessage<'a> {
    #[protocol(var)]
    pub cell: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5944)]
pub struct ObjectGroundRemovedMultipleMessage<'a> {
    #[protocol(var_contents)]
    pub cells: std::borrow::Cow<'a, [u16]>,
}
