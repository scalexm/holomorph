use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(PlayerStatus, 415, status_id| i8);
impl_type!(PlayerStatusExtended, 414, base| PlayerStatus, message| String);
