use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6192)]
pub struct MoodSmileyRequestMessage<'a> {
    #[protocol(var)]
    pub smiley_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 801)]
pub struct ChatSmileyMessage<'a> {
    pub entity_id: f64,
    #[protocol(var)]
    pub smiley_id: u16,
    pub account_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6196)]
pub struct MoodSmileyResultMessage<'a> {
    pub result_code: u8,
    #[protocol(var)]
    pub smiley_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6388)]
pub struct MoodSmileyUpdateMessage<'a> {
    pub account_id: u32,
    #[protocol(var)]
    pub player_id: u64,
    #[protocol(var)]
    pub smiley_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 800)]
pub struct ChatSmileyRequestMessage<'a> {
    #[protocol(var)]
    pub smiley_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6596)]
pub struct ChatSmileyExtraPackListMessage<'a> {
    pub pack_ids: &'a [u8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6185)]
pub struct LocalizedChatSmileyMessage<'a> {
    pub base: ChatSmileyMessage<'a>,
    #[protocol(var)]
    pub cell_id: u16,
}
