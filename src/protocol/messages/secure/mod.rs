use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(TrustStatusMessage, 6267, trusted| Flag, certified| Flag);
