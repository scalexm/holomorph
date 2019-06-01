use crate::types::game::character::characteristic::CharacterCharacteristicsInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6322)]
pub struct FighterStatsListMessage<'a> {
    pub stats: CharacterCharacteristicsInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5670)]
pub struct CharacterLevelUpMessage<'a> {
    #[protocol(var)]
    pub new_level: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5686)]
pub struct LifePointsRegenEndMessage<'a> {
    pub base: UpdateLifePointsMessage<'a>,
    #[protocol(var)]
    pub life_points_gained: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6739)]
pub struct ResetCharacterStatsRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5684)]
pub struct LifePointsRegenBeginMessage<'a> {
    pub regen_rate: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 500)]
pub struct CharacterStatsListMessage<'a> {
    pub stats: CharacterCharacteristicsInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6321)]
pub struct CharacterExperienceGainMessage<'a> {
    #[protocol(var)]
    pub experience_character: u64,
    #[protocol(var)]
    pub experience_mount: u64,
    #[protocol(var)]
    pub experience_guild: u64,
    #[protocol(var)]
    pub experience_incarnation: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6076)]
pub struct CharacterLevelUpInformationMessage<'a> {
    pub base: CharacterLevelUpMessage<'a>,
    pub name: &'a str,
    #[protocol(var)]
    pub id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5658)]
pub struct UpdateLifePointsMessage<'a> {
    #[protocol(var)]
    pub life_points: u32,
    #[protocol(var)]
    pub max_life_points: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
