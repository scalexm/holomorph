use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5691)]
pub struct EmotePlayMassiveMessage<'a> {
    pub base: EmotePlayAbstractMessage<'a>,
    pub actor_ids: std::borrow::Cow<'a, [f64]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5690)]
pub struct EmotePlayAbstractMessage<'a> {
    pub emote_id: u8,
    pub emote_start_time: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5688)]
pub struct EmotePlayErrorMessage<'a> {
    pub emote_id: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5689)]
pub struct EmoteListMessage<'a> {
    pub emote_ids: &'a [u8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5644)]
pub struct EmoteAddMessage<'a> {
    pub emote_id: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5685)]
pub struct EmotePlayRequestMessage<'a> {
    pub emote_id: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5683)]
pub struct EmotePlayMessage<'a> {
    pub base: EmotePlayAbstractMessage<'a>,
    pub actor_id: f64,
    pub account_id: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5687)]
pub struct EmoteRemoveMessage<'a> {
    pub emote_id: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
