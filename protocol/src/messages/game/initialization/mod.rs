use crate::types::game::character::restriction::ActorRestrictionsInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6339)]
pub struct CharacterCapabilitiesMessage<'a> {
    #[protocol(var)]
    pub guild_emblem_symbol_categories: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6471)]
pub struct CharacterLoadingCompleteMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5726)]
pub struct OnConnectionEventMessage<'a> {
    pub event_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 170)]
pub struct SetCharacterRestrictionsMessage<'a> {
    pub actor_id: f64,
    pub restrictions: ActorRestrictionsInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6237)]
pub struct ServerExperienceModificatorMessage<'a> {
    #[protocol(var)]
    pub experience_percent: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
