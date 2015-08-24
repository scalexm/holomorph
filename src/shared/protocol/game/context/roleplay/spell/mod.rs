use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(SpellForgetUIMessage, 5565, open| bool);
impl_type!(SpellForgottenMessage, 5834, spells_id| Vec<VarShort>, boost_point| VarShort);
impl_type!(SpellItemBoostMessage, 6011, stat_id| VarInt, spell_id| VarShort, value| VarShort);
impl_type!(SpellUpgradeFailureMessage, 1202);
impl_type!(SpellUpgradeRequestMessage, 5608, spell_id| VarShort, spell_level| i8);
impl_type!(SpellUpgradeSuccessMessage, 1201, spell_id| i32, spell_level| i8);
impl_type!(ValidateSpellForgetMessage, 1700, spell_id| VarShort);
