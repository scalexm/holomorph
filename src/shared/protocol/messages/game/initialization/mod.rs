use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::types::game::character::restriction::ActorRestrictionsInformations;
impl_type!(CharacterCapabilitiesMessage, 6339, guild_emblem_symbol_categories| VarInt);
impl_type!(CharacterLoadingCompleteMessage, 6471);
impl_type!(OnConnectionEventMessage, 5726, event_type| i8);
impl_type!(ServerExperienceModificatorMessage, 6237, experience_percent| VarShort);
impl_type!(SetCharacterRestrictionsMessage, 170, actor_id| i32, restrictions| ActorRestrictionsInformations);
