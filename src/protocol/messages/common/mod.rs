pub mod basic;
use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(NetworkDataContainerMessage, 2);//, content| Bytes);
