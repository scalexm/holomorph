use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(CinematicMessage, 6053, cinematic_id| VarShort);
impl_type!(URLOpenMessage, 6266, url_id| i8);
