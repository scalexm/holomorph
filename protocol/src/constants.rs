pub mod identification_failure_reason {
    pub const BAD_VERSION: u8 = 1;
    pub const WRONG_CREDENTIALS: u8 = 2;
    pub const BANNED: u8 = 3;
    pub const KICKED: u8 = 4;
    pub const IN_MAINTENANCE: u8 = 5;
    pub const TOO_MANY_ON_IP: u8 = 6;
    pub const TIME_OUT: u8 = 7;
    pub const BAD_IPRANGE: u8 = 8;
    pub const CREDENTIALS_RESET: u8 = 9;
    pub const EMAIL_UNVALIDATED: u8 = 10;
    pub const OTP_TIMEOUT: u8 = 11;
    pub const LOCKED: u8 = 12;
    pub const SERVICE_UNAVAILABLE: u8 = 53;
    pub const EXTERNAL_ACCOUNT_LINK_REFUSED: u8 = 61;
    pub const EXTERNAL_ACCOUNT_ALREADY_LINKED: u8 = 62;
    pub const UNKNOWN_AUTH_ERROR: u8 = 99;
    pub const SPARE: u8 = 100;
}

pub mod server_status {
    pub const UNKNOWN: u8 = 0;
    pub const OFFLINE: u8 = 1;
    pub const STARTING: u8 = 2;
    pub const ONLINE: u8 = 3;
    pub const NOJOIN: u8 = 4;
    pub const SAVING: u8 = 5;
    pub const STOPING: u8 = 6;
    pub const FULL: u8 = 7;
}

pub mod server_connection_error {
    pub const DUE_TO_STATUS: u8 = 0;
    pub const NO_REASON: u8 = 1;
    pub const ACCOUNT_RESTRICTED: u8 = 2;
    pub const COMMUNITY_RESTRICTED: u8 = 3;
    pub const LOCATION_RESTRICTED: u8 = 4;
    pub const SUBSCRIBERS_ONLY: u8 = 5;
    pub const REGULAR_PLAYERS_ONLY: u8 = 6;
    pub const MONOACCOUNT_CANNOT_VERIFY: u8 = 7;
    pub const MONOACCOUNT_ONLY: u8 = 8;
    pub const SERVER_OVERLOAD: u8 = 9;
}
