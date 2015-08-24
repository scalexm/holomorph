use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(AreaFightModificatorUpdateMessage, 6493, spell_pair_id| i32);
