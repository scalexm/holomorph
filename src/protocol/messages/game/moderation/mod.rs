use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(PopupWarningMessage, 6134, lock_duration| i8, author| String, content| String);
