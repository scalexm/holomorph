use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(ContentPart, 350, id| String, state| i8);
