pub mod storage;
pub mod spells;
pub mod preset;
pub mod items;
pub mod exchanges;
use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(KamasUpdateMessage, 5537, kamas_total| VarInt);
impl_type!(ObjectAveragePricesErrorMessage, 6336);
impl_type!(ObjectAveragePricesGetMessage, 6334);
impl_type!(ObjectAveragePricesMessage, 6335, ids| Vec<VarShort>, avg_prices| Vec<VarInt>);
