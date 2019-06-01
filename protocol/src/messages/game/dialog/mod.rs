use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6012)]
pub struct PauseDialogMessage<'a> {
    pub dialog_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5502)]
pub struct LeaveDialogMessage<'a> {
    pub dialog_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5501)]
pub struct LeaveDialogRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
