use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(StartupActionAddMessage, 6538, new_action| StartupActionAddObject);
impl_type!(StartupActionFinishedMessage, 1304, success| Flag, automatic_action| Flag, action_id| i32);
impl_type!(StartupActionsAllAttributionMessage, 6537, character_id| i32);
impl_type!(StartupActionsExecuteMessage, 1302);
impl_type!(StartupActionsListMessage, 1301, actions| Vec<StartupActionAddObject>);
impl_type!(StartupActionsObjetAttributionMessage, 1303, action_id| i32, character_id| i32);
use protocol::game::data::items::ObjectItemInformationWithQuantity; 
impl_type!(StartupActionAddObject, 52, uid| i32, title| String, text| String, desc_url| String, picture_url| String, items| Vec<ObjectItemInformationWithQuantity>);
