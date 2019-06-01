use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6730)]
pub struct ChatCommunityChannelCommunityMessage<'a> {
    pub community_id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6729)]
pub struct ChatCommunityChannelSetCommunityRequestMessage<'a> {
    pub community_id: i16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
