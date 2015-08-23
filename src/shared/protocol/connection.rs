use std::io::{Read, Write};
use io::Result;
use protocol::*;
use protocol::version::*;

impl_type!(HelloConnectMessage, 3,
    salt| String,
    key| VarIntVec<u8>);

impl_type!(IdentificationMessage, 4,
    auto_connect| Flag,
    use_certificate| Flag,
    use_login_token| Flag,
    version| VersionExtended,
    lang| String,
    credentials| VarIntVec<u8>,
    server_id| i16,
    session_optional_salt| i8,
    failed_attempts| Vec<VarUShort>);

impl_type!(IdentificationSuccessMessage, 22,
    has_rights| Flag,
    was_already_connected| Flag,
    login| String,
    nickname| String,
    account_id| i32,
    community_id| i8,
    secret_question| String,
    account_creation| f64,
    subscription_elapsed_duration| f64,
    subscription_end_date| f64);

impl_type!(IdentificationFailedMessage, 20,
    reason| i8);

impl_type!(IdentificationFailedBannedMessage, 6174,
    base| IdentificationFailedMessage,
    ban_end_date| f64);

impl_type!(IdentificationFailedForBadVersionMessage, 21,
    base| IdentificationFailedMessage,
    required_version| Version);

impl_type!(GameServerInformations, 25,
    id| VarUShort,
    status| i8,
    completion| i8,
    is_selectable| bool,
    characters_count| i8,
    date| f64);

impl_type!(ServersListMessage, 30,
    servers| Vec<GameServerInformations>,
    already_connected_to_server_id| VarUShort,
    can_create_new_character| bool);

impl_type!(ServerStatusUpdateMessage, 50,
    server| GameServerInformations);

impl_type!(ServerSelectionMessage, 40,
    server_id| VarUShort);

impl_type!(SelectedServerDataMessage, 42,
    server_id| VarUShort,
    address| String,
    port| u16,
    can_create_new_character| bool,
    ticket| VarIntVec<u8>);

impl_type!(SelectedServerRefusedMessage, 41,
    server_id| VarUShort,
    error| i8,
    server_status| i8);

pub mod identification_failure_reason {
    pub const BAD_VERSION: i8 = 1;
    pub const WRONG_CREDENTIALS: i8 = 2;
    pub const BANNED: i8 = 3;
    pub const KICKED: i8 = 4;
    pub const IN_MAINTENANCE: i8 = 5;
    pub const TOO_MANY_ON_IP: i8 = 6;
    pub const TIME_OUT: i8 = 7;
    pub const BAD_IPRANGE: i8 = 8;
    pub const CREDENTIALS_RESET: i8 = 9;
    pub const EMAIL_UNVALIDATED: i8 = 10;
    pub const OTP_TIMEOUT: i8 = 11;
    pub const SERVICE_UNAVAILABLE: i8 = 53;
    pub const UNKNOWN_AUTH_ERROR: i8 = 99;
    pub const SPARE: i8 = 100;
}

pub mod server_status {
    pub const UNKNOWN: i8 = 0;
    pub const OFFLINE: i8 = 1;
    pub const STARTING: i8 = 2;
    pub const ONLINE: i8 = 3;
    pub const NOJOIN: i8 = 4;
    pub const SAVING: i8 = 5;
    pub const STOPING: i8 = 6;
    pub const FULL: i8 = 7;
}

pub mod server_connection_error {
    pub const DUE_TO_STATUS: i8 = 0;
    pub const NO_REASON: i8 = 1;
    pub const ACCOUNT_RESTRICTED: i8 = 2;
    pub const COMMUNITY_RESTRICTED: i8 = 3;
    pub const LOCATION_RESTRICTED: i8 = 4;
    pub const SUBSCRIBERS_ONLY: i8 = 5;
    pub const REGULAR_PLAYERS_ONLY: i8 = 6;
}
