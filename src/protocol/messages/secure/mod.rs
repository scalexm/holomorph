use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(TrustStatusMessage, 6267, trusted| Flag, certified| Flag);
