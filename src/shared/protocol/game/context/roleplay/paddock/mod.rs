use std::io::{Read, Write};
use io::Result;
use protocol::*;

impl_type!(GameDataPlayFarmObjectAnimationMessage, 6026, cell_id| Vec<VarShort>);
impl_type!(PaddockPropertiesMessage, 5824, properties| PaddockInformationsVariant);
impl_type!(PaddockSellBuyDialogMessage, 6018, bsell| bool, owner_id| VarInt, price| VarInt);
impl_type!(PaddockToSellFilterMessage, 6161, area_id| i32, at_least_nb_mount| i8, at_least_nb_machine| i8, max_price| VarInt);
impl_type!(PaddockToSellListMessage, 6138, page_index| VarShort, total_page| VarShort, paddock_list| Vec<PaddockInformationsForSell>);
impl_type!(PaddockToSellListRequestMessage, 6141, page_index| VarShort);
