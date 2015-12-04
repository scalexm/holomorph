use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(ProtocolRequired, 1, required_version| i32, current_version| i32);
