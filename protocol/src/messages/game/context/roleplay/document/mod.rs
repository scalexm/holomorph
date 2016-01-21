use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(ComicReadingBeginMessage, 6536, comic_id| VarShort);
impl_type!(DocumentReadingBeginMessage, 5675, document_id| VarShort);
