use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(DebugClearHighlightCellsMessage, 2002);
impl_type!(DebugHighlightCellsMessage, 2001, color| i32, cells| Vec<VarShort>);
impl_type!(DebugInClientMessage, 6028, level| i8, message| String);
