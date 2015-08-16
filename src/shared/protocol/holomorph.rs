use std::io::{Read, Write};
use io::Result;
use super::*;

impl_type!(HelloMessage, 1,
    salt| String);

impl_type!(IdentificationMessage, 2,
    id| i16,
    key| String,
    state| i8,
    ip| String,
    port| u16);


impl_type!(StateMessage, 3,
    state| i8);

impl_type!(DisconnectPlayerMessage, 4,
    id| i32);
