pub mod compass;
use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
 use types::game::context::roleplay::AtlasPointsInformations;
impl_type!(AtlasPointInformationsMessage, 5956, type_| AtlasPointsInformations);
