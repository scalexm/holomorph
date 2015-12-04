use std::io::{Read, Write};
use std::io::Result;
use protocol::*;

impl_type!(PurchasableDialogMessage, 5739, buy_or_sell| bool, purchasable_id| VarInt, price| VarInt);
