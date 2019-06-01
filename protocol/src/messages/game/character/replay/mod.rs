use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 167)]
pub struct CharacterReplayRequestMessage<'a> {
    #[protocol(var)]
    pub character_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
