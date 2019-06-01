pub mod compass;

use crate::types::game::context::roleplay::AtlasPointsInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5956)]
pub struct AtlasPointInformationsMessage<'a> {
    pub type_: AtlasPointsInformations<'a>,
}
