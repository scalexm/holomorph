use crate::types::game::data::items::SpellItem;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1200)]
pub struct SpellListMessage<'a> {
    pub spell_previsualization: bool,
    pub spells: std::borrow::Cow<'a, [SpellItem<'a>]>,
}
