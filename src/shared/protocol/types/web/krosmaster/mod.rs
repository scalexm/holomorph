use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(KrosmasterFigure, 397, uid| String, figure| VarShort, pedestal| VarShort, bound| bool);
