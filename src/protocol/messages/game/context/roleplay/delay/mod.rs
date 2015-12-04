use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(GameRolePlayDelayedActionFinishedMessage, 6150, delayed_character_id| i32, delay_type_id| i8);
impl_type!(GameRolePlayDelayedActionMessage, 6153, delayed_character_id| i32, delay_type_id| i8, delay_end_time| f64);
impl_type!(GameRolePlayDelayedObjectUseMessage, 6425, base| GameRolePlayDelayedActionMessage, object_gid| VarShort);
