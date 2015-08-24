use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(BasicPingMessage, 182, quiet| bool);
impl_type!(BasicPongMessage, 183, quiet| bool);
impl_type!(BasicStatMessage, 6530, stat_id| VarShort);

impl_type!(StatisticData, 484, action_id| VarShort);
impl_type!(StatisticDataBoolean, 482, base| StatisticData, value| bool);
impl_type!(StatisticDataByte, 486, base| StatisticData, value| i8);
impl_type!(StatisticDataInt, 485, base| StatisticData, value| i32);
impl_type!(StatisticDataShort, 488, base| StatisticData, value| i16);
impl_type!(StatisticDataString, 487, base| StatisticData, value| String);
