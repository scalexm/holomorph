use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6512)]
pub struct WarnOnPermaDeathMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5996)]
pub struct GameRolePlayPlayerLifeStatusMessage<'a> {
    pub state: u8,
    pub phenix_map_id: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 746)]
pub struct GameRolePlayGameOverMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 745)]
pub struct GameRolePlayFreeSoulRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
