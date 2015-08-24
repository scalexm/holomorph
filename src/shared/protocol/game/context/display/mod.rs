use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(DisplayNumericalValuePaddockMessage, 6563, ride_id| i32, value| i32, ttype| i8);
