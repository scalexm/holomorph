use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(GameRolePlaySpellAnimMessage, 6114, caster_id| VarLong, target_cell_id| VarShort, spell_id| VarShort, spell_level| i8);
