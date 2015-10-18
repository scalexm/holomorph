use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(BasicPingMessage, 182, quiet| bool);
impl_type!(BasicPongMessage, 183, quiet| bool);
impl_type!(BasicStatMessage, 6530, stat_id| VarShort);
