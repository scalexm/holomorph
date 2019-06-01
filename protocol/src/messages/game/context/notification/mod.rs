use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6089)]
pub struct NotificationResetMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6103)]
pub struct NotificationByServerMessage<'a> {
    #[protocol(var)]
    pub id: u16,
    pub parameters: std::borrow::Cow<'a, [&'a str]>,
    pub force_open: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6087)]
pub struct NotificationListMessage<'a> {
    #[protocol(var_contents)]
    pub flags: std::borrow::Cow<'a, [i32]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6090)]
pub struct NotificationUpdateFlagMessage<'a> {
    #[protocol(var)]
    pub index: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
