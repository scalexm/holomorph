use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6275)]
pub struct MailStatusMessage<'a> {
    #[protocol(var)]
    pub unread: u16,
    #[protocol(var)]
    pub total: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6292)]
pub struct NewMailMessage<'a> {
    pub base: MailStatusMessage<'a>,
    pub senders_account_id: std::borrow::Cow<'a, [u32]>,
}
