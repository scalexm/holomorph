use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(TrustCertificate, 377, id| i32, hash| String);
