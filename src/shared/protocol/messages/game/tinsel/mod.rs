use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(OrnamentGainedMessage, 6368, ornament_id| i16);
impl_type!(OrnamentSelectedMessage, 6369, ornament_id| VarShort);
impl_type!(OrnamentSelectErrorMessage, 6370, reason| i8);
impl_type!(OrnamentSelectRequestMessage, 6374, ornament_id| VarShort);
impl_type!(TitleGainedMessage, 6364, title_id| VarShort);
impl_type!(TitleLostMessage, 6371, title_id| VarShort);
impl_type!(TitlesAndOrnamentsListMessage, 6367, titles| Vec<VarShort>, ornaments| Vec<VarShort>, active_title| VarShort, active_ornament| VarShort);
impl_type!(TitlesAndOrnamentsListRequestMessage, 6363);
impl_type!(TitleSelectedMessage, 6366, title_id| VarShort);
impl_type!(TitleSelectErrorMessage, 6373, reason| i8);
impl_type!(TitleSelectRequestMessage, 6365, title_id| VarShort);
