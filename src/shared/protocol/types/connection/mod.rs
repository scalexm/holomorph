use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(GameServerInformations, 25, id| VarShort, status| i8, completion| i8, is_selectable| bool, characters_count| i8, date| f64);
