use crate::types::game::data::items::ObjectItemQuantity;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6290)]
pub struct ObjectFeedMessage<'a> {
    #[protocol(var)]
    pub object_uid: u32,
    pub meal: std::borrow::Cow<'a, [ObjectItemQuantity<'a>]>,
}
