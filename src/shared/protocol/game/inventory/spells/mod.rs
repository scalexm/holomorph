use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::game::data::items::SpellItem;
impl_type!(SpellListMessage, 1200, spell_previsualization| bool, spells| Vec<SpellItem>);
