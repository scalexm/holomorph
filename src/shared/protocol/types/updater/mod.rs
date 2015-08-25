use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(ContentPart, 350, id| String, state| i8);
