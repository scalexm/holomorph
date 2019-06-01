use crate::variants::AcquaintanceInformationVariant;
use crate::variants::FriendInformationsVariant;
use crate::variants::FriendSpouseInformationsVariant;
use crate::variants::IgnoredInformationsVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6159)]
pub struct GuildMemberSetWarnOnConnectionMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6513)]
pub struct WarnOnPermaDeathStateMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6078)]
pub struct FriendWarnOnLevelGainStateMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6819)]
pub struct AcquaintancesGetListMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6823)]
pub struct FriendStatusShareStateMessage<'a> {
    pub share: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5677)]
pub struct IgnoredDeleteResultMessage<'a> {
    #[protocol(flag)]
    pub success: bool,
    #[protocol(flag)]
    pub session: bool,
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5630)]
pub struct FriendWarnOnConnectionStateMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5604)]
pub struct FriendSpouseJoinRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6818)]
pub struct AcquaintanceAddedMessage<'a> {
    pub acquaintance_added: AcquaintanceInformationVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6160)]
pub struct GuildMemberWarnOnConnectionStateMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5678)]
pub struct IgnoredAddedMessage<'a> {
    pub ignore_added: IgnoredInformationsVariant<'a>,
    pub session: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 4002)]
pub struct FriendsListMessage<'a> {
    pub friends_list: std::borrow::Cow<'a, [FriendInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5600)]
pub struct FriendAddFailureMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5674)]
pub struct IgnoredListMessage<'a> {
    pub ignored_list: std::borrow::Cow<'a, [IgnoredInformationsVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5601)]
pub struct FriendDeleteResultMessage<'a> {
    pub success: bool,
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5924)]
pub struct FriendUpdateMessage<'a> {
    pub friend_updated: FriendInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 4004)]
pub struct FriendAddRequestMessage<'a> {
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5599)]
pub struct FriendAddedMessage<'a> {
    pub friend_added: FriendInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 4001)]
pub struct FriendsGetListMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5673)]
pub struct IgnoredAddRequestMessage<'a> {
    pub name: &'a str,
    pub session: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5603)]
pub struct FriendDeleteRequestMessage<'a> {
    pub account_id: u32,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5602)]
pub struct FriendSetWarnOnConnectionMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5679)]
pub struct IgnoredAddFailureMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6822)]
pub struct FriendSetStatusShareMessage<'a> {
    pub share: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6355)]
pub struct SpouseGetInformationsMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5605)]
pub struct FriendJoinRequestMessage<'a> {
    pub name: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6821)]
pub struct ContactAddFailureMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6077)]
pub struct FriendSetWarnOnLevelGainMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6820)]
pub struct AcquaintancesListMessage<'a> {
    pub acquaintance_list: std::borrow::Cow<'a, [AcquaintanceInformationVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6265)]
pub struct SpouseStatusMessage<'a> {
    pub has_spouse: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5680)]
pub struct IgnoredDeleteRequestMessage<'a> {
    pub account_id: u32,
    pub session: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6356)]
pub struct SpouseInformationsMessage<'a> {
    pub spouse: FriendSpouseInformationsVariant<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5606)]
pub struct FriendSpouseFollowWithCompassRequestMessage<'a> {
    pub enable: bool,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 5676)]
pub struct IgnoredGetListMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}
