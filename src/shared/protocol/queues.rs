use std::io::{Read, Write};
use io::Result;
use super::*;

impl_type!(LoginQueueStatusMessage, 10,
    position| u16,
    total| u16);

impl_type!(QueueStatusMessage, 6100,
    position| u16,
    total| u16);
