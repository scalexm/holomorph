use crate::types::game::look::EntityLook;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5935)]
pub struct ContactLookRequestByIdMessage<'a> {
    pub base: ContactLookRequestMessage<'a>,
    #[protocol(var)]
    pub player_id: u64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6688)]
pub struct SocialNoticeMessage<'a> {
    pub content: &'a str,
    pub timestamp: u32,
    #[protocol(var)]
    pub member_id: u64,
    pub member_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5933)]
pub struct ContactLookRequestByNameMessage<'a> {
    pub base: ContactLookRequestMessage<'a>,
    pub player_name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6045)]
pub struct ContactLookErrorMessage<'a> {
    #[protocol(var)]
    pub request_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6684)]
pub struct SocialNoticeSetErrorMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5932)]
pub struct ContactLookRequestMessage<'a> {
    pub request_id: u8,
    pub contact_type: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6686)]
pub struct SocialNoticeSetRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5934)]
pub struct ContactLookMessage<'a> {
    #[protocol(var)]
    pub request_id: u32,
    pub player_name: &'a str,
    #[protocol(var)]
    pub player_id: u64,
    pub look: EntityLook<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6695)]
pub struct BulletinMessage<'a> {
    pub base: SocialNoticeMessage<'a>,
    pub last_notified_timestamp: u32,
}
