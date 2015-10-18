use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(LoginQueueStatusMessage, 10, position| i16, total| i16);
impl_type!(QueueStatusMessage, 6100, position| i16, total| i16);
