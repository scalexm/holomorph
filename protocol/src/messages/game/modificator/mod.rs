use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(AreaFightModificatorUpdateMessage, 6493, spell_pair_id| i32);
