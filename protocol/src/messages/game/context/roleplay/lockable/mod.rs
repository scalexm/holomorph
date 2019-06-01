use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5666)]
pub struct LockableChangeCodeMessage<'a> {
    pub code: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5672)]
pub struct LockableCodeResultMessage<'a> {
    pub result: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5667)]
pub struct LockableUseCodeMessage<'a> {
    pub code: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5740)]
pub struct LockableShowCodeDialogMessage<'a> {
    pub change_or_use: bool,
    pub code_size: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5668)]
pub struct LockableStateUpdateHouseDoorMessage<'a> {
    pub base: LockableStateUpdateAbstractMessage<'a>,
    #[protocol(var)]
    pub house_id: u32,
    pub instance_id: u32,
    pub second_hand: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5669)]
pub struct LockableStateUpdateStorageMessage<'a> {
    pub base: LockableStateUpdateAbstractMessage<'a>,
    pub map_id: f64,
    #[protocol(var)]
    pub element_id: u32,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5671)]
pub struct LockableStateUpdateAbstractMessage<'a> {
    pub locked: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
