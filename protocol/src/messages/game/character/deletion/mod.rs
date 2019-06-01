use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 165)]
pub struct CharacterDeletionRequestMessage<'a> {
    #[protocol(var)]
    pub character_id: u64,
    pub secret_answer_hash: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 166)]
pub struct CharacterDeletionErrorMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
