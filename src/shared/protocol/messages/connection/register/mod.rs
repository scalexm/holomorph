use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(NicknameAcceptedMessage, 5641);
impl_type!(NicknameChoiceRequestMessage, 5639, nickname| String);
impl_type!(NicknameRefusedMessage, 5638, reason| i8);
impl_type!(NicknameRegistrationMessage, 5640);
