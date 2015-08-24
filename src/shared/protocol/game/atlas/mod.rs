use std::io::{Read, Write};
use io::Result;
use protocol::*;
use protocol::game::context::roleplay::AtlasPointsInformations; 
impl_type!(AtlasPointInformationsMessage, 5956, ttype| AtlasPointsInformations);
