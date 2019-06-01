use crate::types::game::entity::EntityInformation;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6771)]
pub struct EntityInformationMessage<'a> {
    pub entity: EntityInformation<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6775)]
pub struct EntitiesInformationMessage<'a> {
    pub entities: std::borrow::Cow<'a, [EntityInformation<'a>]>,
}
