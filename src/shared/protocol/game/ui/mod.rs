use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(ClientUIOpenedByObjectMessage, 6463, base| ClientUIOpenedMessage, uid| VarInt);
impl_type!(ClientUIOpenedMessage, 6459, ttype| i8);
