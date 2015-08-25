use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(CharacterReplayRequestMessage, 167, character_id| i32);
