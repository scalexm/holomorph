use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(CharacterDeletionErrorMessage, 166, reason| i8);
impl_type!(CharacterDeletionRequestMessage, 165, character_id| VarLong, secret_answer_hash| String);
