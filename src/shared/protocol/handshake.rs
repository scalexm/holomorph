use std::io::{Read, Write};
use ::io::{ReadExt, WriteExt, Result};
use super::*;

impl_type!(ProtocolRequired, 1,
    required_version| i32,
    current_version| i32);
