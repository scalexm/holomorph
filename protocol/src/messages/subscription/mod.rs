use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(SubscriptionUpdateMessage, 6616, timestamp| f64);
