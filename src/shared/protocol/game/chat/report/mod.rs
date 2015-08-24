use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(ChatMessageReportMessage, 821, sender_name| String, content| String, timestamp| i32, channel| i8, fingerprint| String, reason| i8);
