use crate::messages::game::character::replay::CharacterReplayRequestMessage;
use crate::types::game::character::choice::CharacterBaseInformations;
use crate::types::game::character::choice::CharacterToRemodelInformations;
use crate::types::game::character::choice::RemodelingInformation;
use crate::variants::CharacterBaseInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5545)]
pub struct CharactersListErrorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6068)]
pub struct CharacterSelectedForceMessage<'a> {
    pub id: i32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 150)]
pub struct CharactersListRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6072)]
pub struct CharacterSelectedForceReadyMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6550)]
pub struct CharactersListWithRemodelingMessage<'a> {
    pub base: CharactersListMessage<'a>,
    pub characters_to_remodel: std::borrow::Cow<'a, [CharacterToRemodelInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6084)]
pub struct CharacterFirstSelectionMessage<'a> {
    pub base: CharacterSelectionMessage<'a>,
    pub do_tutorial: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 152)]
pub struct CharacterSelectionMessage<'a> {
    #[protocol(var)]
    pub id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6549)]
pub struct CharacterSelectionWithRemodelMessage<'a> {
    pub base: CharacterSelectionMessage<'a>,
    pub remodel: RemodelingInformation<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6551)]
pub struct CharacterReplayWithRemodelRequestMessage<'a> {
    pub base: CharacterReplayRequestMessage<'a>,
    pub remodel: RemodelingInformation<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6475)]
pub struct BasicCharactersListMessage<'a> {
    pub characters: std::borrow::Cow<'a, [CharacterBaseInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 151)]
pub struct CharactersListMessage<'a> {
    pub base: BasicCharactersListMessage<'a>,
    pub has_startup_actions: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 153)]
pub struct CharacterSelectedSuccessMessage<'a> {
    pub infos: CharacterBaseInformations<'a>,
    pub is_collecting_stats: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5836)]
pub struct CharacterSelectedErrorMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
