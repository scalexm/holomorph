use crate::messages::game::inventory::items::InventoryContentMessage;
use crate::types::game::data::items::ObjectItem;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5646)]
pub struct StorageInventoryContentMessage<'a> {
    pub base: InventoryContentMessage<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6036)]
pub struct StorageObjectsUpdateMessage<'a> {
    pub object_list: std::borrow::Cow<'a, [ObjectItem<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6035)]
pub struct StorageObjectsRemoveMessage<'a> {
    #[protocol(var_contents)]
    pub object_uid_list: std::borrow::Cow<'a, [u32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5645)]
pub struct StorageKamasUpdateMessage<'a> {
    #[protocol(var)]
    pub kamas_total: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5648)]
pub struct StorageObjectRemoveMessage<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5647)]
pub struct StorageObjectUpdateMessage<'a> {
    pub object: ObjectItem<'a>,
}
