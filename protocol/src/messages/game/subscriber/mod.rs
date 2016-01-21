use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(SubscriptionLimitationMessage, 5542, reason| i8);
impl_type!(SubscriptionZoneMessage, 5573, active| bool);
