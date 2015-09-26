use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::messages::debug::DebugInClientMessage;
impl_type!(ClientYouAreDrunkMessage, 6594, base| DebugInClientMessage);
