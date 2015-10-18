use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(GameServerInformations, 25, id| VarShort, type_| i8, status| i8, completion| i8, is_selectable| bool, characters_count| i8, characters_slots| i8, date| f64);
