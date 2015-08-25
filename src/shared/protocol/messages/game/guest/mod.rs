use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(GuestLimitationMessage, 6506, reason| i8);
impl_type!(GuestModeMessage, 6505, active| bool);
