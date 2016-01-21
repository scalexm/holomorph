use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
 use variants::StatisticDataVariant;
impl_type!(BasicPingMessage, 182, quiet| bool);
impl_type!(BasicPongMessage, 183, quiet| bool);
impl_type!(BasicStatMessage, 6530, stat_id| VarShort);
impl_type!(BasicStatWithDataMessage, 6573, base| BasicStatMessage, datas| Vec<StatisticDataVariant>);
