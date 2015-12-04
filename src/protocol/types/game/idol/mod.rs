use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(Idol, 489, id| VarShort, xp_bonus_percent| VarShort, drop_bonus_percent| VarShort);
impl_type!(PartyIdol, 490, base| Idol, owners_ids| Vec<i32>);
