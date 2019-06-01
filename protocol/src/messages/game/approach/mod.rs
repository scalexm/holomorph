use crate::variants::ServerSessionConstantVariant;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 110)]
pub struct AuthenticationTicketMessage<'a> {
    pub lang: &'a str,
    pub ticket: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6434)]
pub struct ServerSessionConstantsMessage<'a> {
    pub variables: std::borrow::Cow<'a, [ServerSessionConstantVariant<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 112)]
pub struct AuthenticationTicketRefusedMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6216)]
pub struct AccountCapabilitiesMessage<'a> {
    #[protocol(flag)]
    pub tutorial_available: bool,
    #[protocol(flag)]
    pub can_create_new_character: bool,
    pub account_id: u32,
    #[protocol(var)]
    pub breeds_visible: u32,
    #[protocol(var)]
    pub breeds_available: u32,
    pub status: i8,
    pub unlimited_restat_end_date: f64,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 101)]
pub struct HelloGameMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 111)]
pub struct AuthenticationTicketAcceptedMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6305)]
pub struct ServerOptionalFeaturesMessage<'a> {
    pub features: &'a [u8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6540)]
pub struct ReloginTokenRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6029)]
pub struct AccountLoggingKickedMessage<'a> {
    #[protocol(var)]
    pub days: u16,
    pub hours: u8,
    pub minutes: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 109)]
pub struct AlreadyConnectedMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6539)]
pub struct ReloginTokenStatusMessage<'a> {
    pub valid_token: bool,
    #[protocol(var)]
    pub ticket: &'a [i8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6340)]
pub struct ServerSettingsMessage<'a> {
    #[protocol(flag)]
    pub is_mono_account: bool,
    #[protocol(flag)]
    pub has_free_autopilot: bool,
    pub lang: &'a str,
    pub community: u8,
    pub game_type: i8,
    #[protocol(var)]
    pub arena_leave_ban_time: u16,
    pub item_max_level: u32,
}
