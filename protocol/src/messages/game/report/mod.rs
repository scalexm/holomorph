use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(CharacterReportMessage, 6079, reported_id| VarLong, reason| i8);
