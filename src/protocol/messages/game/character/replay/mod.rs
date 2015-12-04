use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(CharacterReplayRequestMessage, 167, character_id| i32);
