use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::types::game::look::EntityLook;
impl_type!(AccessoryPreviewErrorMessage, 6521, error| i8);
impl_type!(AccessoryPreviewMessage, 6517, look| EntityLook);
impl_type!(AccessoryPreviewRequestMessage, 6518, generic_id| Vec<VarShort>);
