pub mod alignment;
pub mod characteristic;
pub mod choice;
pub mod restriction;
pub mod status;

use crate::types::game::context::roleplay::BasicAllianceInformations;
use crate::types::game::context::roleplay::BasicGuildInformations;
use crate::types::game::look::EntityLook;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 193)]
pub struct CharacterMinimalPlusLookAndGradeInformations<'a> {
    pub base: CharacterMinimalPlusLookInformations<'a>,
    #[protocol(var)]
    pub grade: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 556)]
pub struct CharacterMinimalGuildPublicInformations<'a> {
    pub base: CharacterMinimalInformations<'a>,
    #[protocol(var)]
    pub rank: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 400)]
pub struct AbstractCharacterInformation<'a> {
    #[protocol(var)]
    pub id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 445)]
pub struct CharacterMinimalGuildInformations<'a> {
    pub base: CharacterMinimalPlusLookInformations<'a>,
    pub guild: BasicGuildInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 503)]
pub struct CharacterBasicMinimalInformations<'a> {
    pub base: AbstractCharacterInformation<'a>,
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 163)]
pub struct CharacterMinimalPlusLookInformations<'a> {
    pub base: CharacterMinimalInformations<'a>,
    pub entity_look: EntityLook<'a>,
    pub breed: i8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 444)]
pub struct CharacterMinimalAllianceInformations<'a> {
    pub base: CharacterMinimalGuildInformations<'a>,
    pub alliance: BasicAllianceInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 110)]
pub struct CharacterMinimalInformations<'a> {
    pub base: CharacterBasicMinimalInformations<'a>,
    #[protocol(var)]
    pub level: u16,
}
