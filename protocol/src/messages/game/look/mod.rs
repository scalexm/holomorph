use crate::types::game::look::EntityLook;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6518)]
pub struct AccessoryPreviewRequestMessage<'a> {
    #[protocol(var_contents)]
    pub generic_id: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6517)]
pub struct AccessoryPreviewMessage<'a> {
    pub look: EntityLook<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6521)]
pub struct AccessoryPreviewErrorMessage<'a> {
    pub error: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
