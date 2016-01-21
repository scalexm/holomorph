use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(SequenceEndMessage, 956, action_id| VarShort, author_id| f64, sequence_type| i8);
impl_type!(SequenceStartMessage, 955, sequence_type| i8, author_id| f64);
