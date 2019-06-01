use crate::types::game::character::CharacterBasicMinimalInformations;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6700)]
pub struct ArenaFighterLeaveMessage<'a> {
    pub leaver: CharacterBasicMinimalInformations<'a>,
}
