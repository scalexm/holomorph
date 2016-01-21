use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(ClientUIOpenedByObjectMessage, 6463, base| ClientUIOpenedMessage, uid| VarInt);
impl_type!(ClientUIOpenedMessage, 6459, type_| i8);
