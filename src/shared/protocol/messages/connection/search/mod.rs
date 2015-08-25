use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(AcquaintanceSearchErrorMessage, 6143, reason| i8);
impl_type!(AcquaintanceSearchMessage, 6144, nickname| String);
impl_type!(AcquaintanceServerListMessage, 6142, servers| Vec<VarShort>);
