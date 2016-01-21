use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(SystemMessageDisplayMessage, 189, hang_up| bool, msg_id| VarShort, parameters| Vec<String>);
