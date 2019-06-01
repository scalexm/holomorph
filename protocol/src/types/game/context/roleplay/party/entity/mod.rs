use crate::types::game::look::EntityLook;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 550)]
pub struct PartyEntityMemberInformation<'a> {
    pub base: PartyEntityBaseInformation<'a>,
    #[protocol(var)]
    pub initiative: u16,
    #[protocol(var)]
    pub life_points: u32,
    #[protocol(var)]
    pub max_life_points: u32,
    #[protocol(var)]
    pub prospecting: u16,
    pub regen_rate: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 552)]
pub struct PartyEntityBaseInformation<'a> {
    pub index_id: u8,
    pub entity_model_id: u8,
    pub entity_look: EntityLook<'a>,
}
