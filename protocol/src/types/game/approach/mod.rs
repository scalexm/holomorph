use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(ServerSessionConstant, 430, id| VarShort);
impl_type!(ServerSessionConstantInteger, 433, base| ServerSessionConstant, value| i32);
impl_type!(ServerSessionConstantLong, 429, base| ServerSessionConstant, value| f64);
impl_type!(ServerSessionConstantString, 436, base| ServerSessionConstant, value| String);
