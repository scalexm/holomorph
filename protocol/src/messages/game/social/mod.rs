use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
 use types::game::look::EntityLook;
impl_type!(ContactLookErrorMessage, 6045, request_id| VarInt);
impl_type!(ContactLookMessage, 5934, request_id| VarInt, player_name| String, player_id| VarLong, look| EntityLook);
impl_type!(ContactLookRequestByIdMessage, 5935, base| ContactLookRequestMessage, player_id| VarLong);
impl_type!(ContactLookRequestByNameMessage, 5933, base| ContactLookRequestMessage, player_name| String);
impl_type!(ContactLookRequestMessage, 5932, request_id| i8, contact_type| i8);