use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6425)]
pub struct GameRolePlayDelayedObjectUseMessage<'a> {
    pub base: GameRolePlayDelayedActionMessage<'a>,
    #[protocol(var)]
    pub object_gid: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6153)]
pub struct GameRolePlayDelayedActionMessage<'a> {
    pub delayed_character_id: f64,
    pub delay_type_id: u8,
    pub delay_end_time: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6150)]
pub struct GameRolePlayDelayedActionFinishedMessage<'a> {
    pub delayed_character_id: f64,
    pub delay_type_id: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
