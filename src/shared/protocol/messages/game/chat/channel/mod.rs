use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(ChannelEnablingChangeMessage, 891, channel| i8, enable| bool);
impl_type!(ChannelEnablingMessage, 890, channel| i8, enable| bool);
impl_type!(EnabledChannelsMessage, 892, channels| Vec<u8>, disallowed| Vec<u8>);
