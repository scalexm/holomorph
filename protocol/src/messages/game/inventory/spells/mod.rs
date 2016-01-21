use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
 use types::game::data::items::SpellItem;
impl_type!(SpellListMessage, 1200, spell_previsualization| bool, spells| Vec<SpellItem>);