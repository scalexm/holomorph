use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(AdminCommandMessage, 76, content| String);
impl_type!(AdminQuietCommandMessage, 5662, base| AdminCommandMessage);
impl_type!(ConsoleCommandsListMessage, 6127, aliases| Vec<String>, args| Vec<String>, descriptions| Vec<String>);
impl_type!(ConsoleMessage, 75, type_| i8, content| String);
