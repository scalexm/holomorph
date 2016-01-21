use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(LeaveDialogMessage, 5502, dialog_type| i8);
impl_type!(LeaveDialogRequestMessage, 5501);
impl_type!(PauseDialogMessage, 6012, dialog_type| i8);
