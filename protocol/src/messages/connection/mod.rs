pub mod register;
pub mod search;

use crate::types::connection::GameServerInformations;
use crate::types::version::Version;
use crate::types::version::VersionExtended;
use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6314)]
pub struct CredentialsAcknowledgementMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6209)]
pub struct IdentificationSuccessWithLoginTokenMessage<'a> {
    pub base: IdentificationSuccessMessage<'a>,
    pub login_token: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 3)]
pub struct HelloConnectMessage<'a> {
    pub salt: &'a str,
    #[protocol(var)]
    pub key: &'a [i8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6119)]
pub struct IdentificationAccountForceMessage<'a> {
    pub base: IdentificationMessage<'a>,
    pub forced_account_login: &'a str,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 40)]
pub struct ServerSelectionMessage<'a> {
    #[protocol(var)]
    pub server_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 50)]
pub struct ServerStatusUpdateMessage<'a> {
    pub server: GameServerInformations<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 21)]
pub struct IdentificationFailedForBadVersionMessage<'a> {
    pub base: IdentificationFailedMessage<'a>,
    pub required_version: Version<'a>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6469)]
pub struct SelectedServerDataExtendedMessage<'a> {
    pub base: SelectedServerDataMessage<'a>,
    pub servers: std::borrow::Cow<'a, [GameServerInformations<'a>]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 4)]
pub struct IdentificationMessage<'a> {
    #[protocol(flag)]
    pub autoconnect: bool,
    #[protocol(flag)]
    pub use_certificate: bool,
    #[protocol(flag)]
    pub use_login_token: bool,
    pub version: VersionExtended<'a>,
    pub lang: &'a str,
    #[protocol(var)]
    pub credentials: &'a [i8],
    pub server_id: i16,
    #[protocol(var)]
    pub session_optional_salt: i64,
    #[protocol(var_contents)]
    pub failed_attempts: std::borrow::Cow<'a, [u16]>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 22)]
pub struct IdentificationSuccessMessage<'a> {
    #[protocol(flag)]
    pub has_rights: bool,
    #[protocol(flag)]
    pub was_already_connected: bool,
    pub login: &'a str,
    pub nickname: &'a str,
    pub account_id: u32,
    pub community_id: u8,
    pub secret_question: &'a str,
    pub account_creation: f64,
    pub subscription_elapsed_duration: f64,
    pub subscription_end_date: f64,
    pub havenbag_available_room: u8,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 30)]
pub struct ServersListMessage<'a> {
    pub servers: std::borrow::Cow<'a, [GameServerInformations<'a>]>,
    #[protocol(var)]
    pub already_connected_to_server_id: u16,
    pub can_create_new_character: bool,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6174)]
pub struct IdentificationFailedBannedMessage<'a> {
    pub base: IdentificationFailedMessage<'a>,
    pub ban_end_date: f64,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 41)]
pub struct SelectedServerRefusedMessage<'a> {
    #[protocol(var)]
    pub server_id: u16,
    pub error: u8,
    pub server_status: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 42)]
pub struct SelectedServerDataMessage<'a> {
    #[protocol(var)]
    pub server_id: u16,
    pub address: &'a str,
    pub ports: std::borrow::Cow<'a, [u32]>,
    pub can_create_new_character: bool,
    #[protocol(var)]
    pub ticket: &'a [i8],
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 20)]
pub struct IdentificationFailedMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6731)]
pub struct MigratedServerListMessage<'a> {
    #[protocol(var_contents)]
    pub migrated_server_ids: std::borrow::Cow<'a, [u16]>,
}
