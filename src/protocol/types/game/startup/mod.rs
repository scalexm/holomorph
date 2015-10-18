use std::io::{Read, Write};
use io::Result;
use protocol::*;
use types::game::data::items::ObjectItemInformationWithQuantity; 
impl_type!(StartupActionAddObject, 52, uid| i32, title| String, text| String, desc_url| String, picture_url| String, items| Vec<ObjectItemInformationWithQuantity>);
