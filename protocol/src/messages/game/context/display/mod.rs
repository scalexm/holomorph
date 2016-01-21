use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(DisplayNumericalValuePaddockMessage, 6563, ride_id| i32, value| i32, type_| i8);
