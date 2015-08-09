use std::io::{Read, Write};
use io::Result;
use super::*;

impl_type!(HelloConnectMessage, 3, salt| String, key| VarIntVec<u8>);
