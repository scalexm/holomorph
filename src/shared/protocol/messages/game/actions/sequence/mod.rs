use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(SequenceEndMessage, 956, action_id| VarShort, author_id| i32, sequence_type| i8);
impl_type!(SequenceStartMessage, 955, sequence_type| i8, author_id| i32);
