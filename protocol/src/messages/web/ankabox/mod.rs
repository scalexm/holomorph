use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(MailStatusMessage, 6275, unread| VarShort, total| VarShort);
impl_type!(NewMailMessage, 6292, base| MailStatusMessage, senders_account_id| Vec<i32>);
