use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(AlmanachCalendarDateMessage, 6341, date| i32);
