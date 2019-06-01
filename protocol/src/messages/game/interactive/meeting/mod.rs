use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6293)]
pub struct TeleportToBuddyAnswerMessage<'a> {
    #[protocol(var)]
    pub dungeon_id: u16,
    #[protocol(var)]
    pub buddy_id: u64,
    pub accept: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6302)]
pub struct TeleportBuddiesRequestedMessage<'a> {
    #[protocol(var)]
    pub dungeon_id: u16,
    #[protocol(var)]
    pub inviter_id: u64,
    #[protocol(var_contents)]
    pub invalid_buddies_ids: std::borrow::Cow<'a, [u64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6287)]
pub struct TeleportToBuddyOfferMessage<'a> {
    #[protocol(var)]
    pub dungeon_id: u16,
    #[protocol(var)]
    pub buddy_id: u64,
    #[protocol(var)]
    pub time_left: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6303)]
pub struct TeleportToBuddyCloseMessage<'a> {
    #[protocol(var)]
    pub dungeon_id: u16,
    #[protocol(var)]
    pub buddy_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6289)]
pub struct TeleportBuddiesMessage<'a> {
    #[protocol(var)]
    pub dungeon_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6294)]
pub struct TeleportBuddiesAnswerMessage<'a> {
    pub accept: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
