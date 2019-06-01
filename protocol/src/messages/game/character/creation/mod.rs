use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 161)]
pub struct CharacterCreationResultMessage<'a> {
    pub result: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5544)]
pub struct CharacterNameSuggestionSuccessMessage<'a> {
    pub suggestion: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 160)]
pub struct CharacterCreationRequestMessage<'a> {
    pub name: &'a str,
    pub breed: i8,
    pub sex: bool,
    pub colors: std::borrow::Cow<'a, [i32]>,
    #[protocol(var)]
    pub cosmetic_id: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 162)]
pub struct CharacterNameSuggestionRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 164)]
pub struct CharacterNameSuggestionFailureMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6733)]
pub struct CharacterCanBeCreatedResultMessage<'a> {
    pub yes_you_can: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6732)]
pub struct CharacterCanBeCreatedRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
