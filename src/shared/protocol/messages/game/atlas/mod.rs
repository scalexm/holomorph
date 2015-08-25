pub mod compass;
use std::io::{Read, Write};
use io::Result;
use protocol::*;
 use protocol::types::game::context::roleplay::AtlasPointsInformations;
impl_type!(AtlasPointInformationsMessage, 5956, type_| AtlasPointsInformations);
