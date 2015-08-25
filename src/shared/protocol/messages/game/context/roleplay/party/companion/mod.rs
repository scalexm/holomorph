use std::io::{Read, Write};
use io::Result;
use protocol::*;
use protocol::messages::game::context::roleplay::party::PartyUpdateLightMessage; 
impl_type!(PartyCompanionUpdateLightMessage, 6472, base| PartyUpdateLightMessage, index_id| i8);
