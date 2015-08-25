use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(CharacterDeletionErrorMessage, 166, reason| i8);
impl_type!(CharacterDeletionRequestMessage, 165, character_id| i32, secret_answer_hash| String);
