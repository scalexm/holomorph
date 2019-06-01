use crate::types::game::character::AbstractCharacterInformation;
use crate::types::game::character::CharacterMinimalPlusLookInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 479)]
pub struct CharacterRemodelingInformation<'a> {
    pub base: AbstractCharacterInformation<'a>,
    pub name: &'a str,
    pub breed: i8,
    pub sex: bool,
    #[protocol(var)]
    pub cosmetic_id: u16,
    pub colors: std::borrow::Cow<'a, [i32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 474)]
pub struct CharacterHardcoreOrEpicInformations<'a> {
    pub base: CharacterBaseInformations<'a>,
    pub death_state: u8,
    #[protocol(var)]
    pub death_count: u16,
    #[protocol(var)]
    pub death_max_level: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 480)]
pub struct RemodelingInformation<'a> {
    pub name: &'a str,
    pub breed: i8,
    pub sex: bool,
    #[protocol(var)]
    pub cosmetic_id: u16,
    pub colors: std::borrow::Cow<'a, [i32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 45)]
pub struct CharacterBaseInformations<'a> {
    pub base: CharacterMinimalPlusLookInformations<'a>,
    pub sex: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 477)]
pub struct CharacterToRemodelInformations<'a> {
    pub base: CharacterRemodelingInformation<'a>,
    pub possible_change_mask: u8,
    pub mandatory_change_mask: u8,
}
