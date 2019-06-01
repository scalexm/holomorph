use crate::types::game::data::items::ObjectItemInformationWithQuantity;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 52)]
pub struct StartupActionAddObject<'a> {
    pub uid: u32,
    pub title: &'a str,
    pub text: &'a str,
    pub desc_url: &'a str,
    pub picture_url: &'a str,
    pub items: std::borrow::Cow<'a, [ObjectItemInformationWithQuantity<'a>]>,
}
