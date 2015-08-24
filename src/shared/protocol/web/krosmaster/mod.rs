use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(KrosmasterAuthTokenErrorMessage, 6345, reason| i8);
impl_type!(KrosmasterAuthTokenMessage, 6351, token| String);
impl_type!(KrosmasterAuthTokenRequestMessage, 6346);
impl_type!(KrosmasterInventoryErrorMessage, 6343, reason| i8);
impl_type!(KrosmasterInventoryMessage, 6350, figures| Vec<KrosmasterFigure>);
impl_type!(KrosmasterInventoryRequestMessage, 6344);
impl_type!(KrosmasterPlayingStatusMessage, 6347, playing| bool);
impl_type!(KrosmasterTransferMessage, 6348, uid| String, failure| i8);
impl_type!(KrosmasterTransferRequestMessage, 6349, uid| String);

impl_type!(KrosmasterFigure, 397, uid| String, figure| VarShort, pedestal| VarShort, bound| bool);
