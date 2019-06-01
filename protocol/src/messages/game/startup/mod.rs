use crate::types::game::startup::StartupActionAddObject;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1301)]
pub struct StartupActionsListMessage<'a> {
    pub actions: std::borrow::Cow<'a, [StartupActionAddObject<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6537)]
pub struct StartupActionsAllAttributionMessage<'a> {
    #[protocol(var)]
    pub character_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1302)]
pub struct StartupActionsExecuteMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1303)]
pub struct StartupActionsObjetAttributionMessage<'a> {
    pub action_id: u32,
    #[protocol(var)]
    pub character_id: u64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6538)]
pub struct StartupActionAddMessage<'a> {
    pub new_action: StartupActionAddObject<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 1304)]
pub struct StartupActionFinishedMessage<'a> {
    #[protocol(flag)]
    pub success: bool,
    #[protocol(flag)]
    pub automatic_action: bool,
    pub action_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
