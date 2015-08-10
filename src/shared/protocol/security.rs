use std::io::{Read, Write};
use io::Result;
use super::*;

impl_type!(RawDataMessage, 6253,
    content| VarIntVec<u8>);
