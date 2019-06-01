use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 25)]
pub struct GameServerInformations<'a> {
    #[protocol(flag)]
    pub is_mono_account: bool,
    #[protocol(flag)]
    pub is_selectable: bool,
    #[protocol(var)]
    pub id: u16,
    pub type_: i8,
    pub status: u8,
    pub completion: u8,
    pub characters_count: u8,
    pub characters_slots: u8,
    pub date: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
