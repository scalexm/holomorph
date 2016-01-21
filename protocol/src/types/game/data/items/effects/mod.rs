use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(ObjectEffect, 76, action_id| VarShort);
impl_type!(ObjectEffectCreature, 71, base| ObjectEffect, monster_family_id| VarShort);
impl_type!(ObjectEffectDate, 72, base| ObjectEffect, year| VarShort, month| i8, day| i8, hour| i8, minute| i8);
impl_type!(ObjectEffectDice, 73, base| ObjectEffect, dice_num| VarShort, dice_side| VarShort, dice_const| VarShort);
impl_type!(ObjectEffectDuration, 75, base| ObjectEffect, days| VarShort, hours| i8, minutes| i8);
impl_type!(ObjectEffectInteger, 70, base| ObjectEffect, value| VarShort);
impl_type!(ObjectEffectLadder, 81, base| ObjectEffectCreature, monster_count| VarInt);
impl_type!(ObjectEffectMinMax, 82, base| ObjectEffect, min| VarInt, max| VarInt);
impl_type!(ObjectEffectMount, 179, base| ObjectEffect, mount_id| i32, date| f64, model_id| VarShort);
impl_type!(ObjectEffectString, 74, base| ObjectEffect, value| String);
