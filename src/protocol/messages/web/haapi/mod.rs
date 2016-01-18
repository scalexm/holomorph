use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(HaapiApiKeyMessage, 6649, return_type| i8, key_type| i8, token| String);
impl_type!(HaapiApiKeyRequestMessage, 6648, key_type| i8);
