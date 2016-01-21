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

pub mod chat_channels_multi {
    pub const GLOBAL: i8 = 0;
    pub const TEAM: i8 = 1;
    pub const GUILD: i8 = 2;
    pub const ALLIANCE: i8 = 3;
    pub const PARTY: i8 = 4;
    pub const SALES: i8 = 5;
    pub const SEEK: i8 = 6;
    pub const NOOB: i8 = 7;
    pub const ADMIN: i8 = 8;
    pub const ADS: i8 = 12;
    pub const ARENA: i8 = 13;
}

pub mod text_information_type {
    pub const MESSAGE: i8 = 0;
    pub const ERROR: i8 = 1;
    pub const PVP: i8 = 2;
    pub const FIGHT_LOG: i8 = 3;
    pub const POPUP: i8 = 4;
    pub const LIVING_OBJECT: i8 = 5;
    pub const ENTITY_TALK: i8 = 6;
    pub const FIGHT: i8 = 7;
}

pub mod player_status {
    pub const OFFLINE: i8 = 0;
    pub const UNKNOWN: i8 = 1;
    pub const AVAILABLE: i8 = 10;
    pub const IDLE: i8 = 20;
    pub const AFK: i8 = 21;
    pub const PRIVATE: i8 = 30;
    pub const SOLO: i8 = 40;
}

pub mod chat_error {
    pub const UNKNOWN: i8 = 0;
    pub const RECEIVER_NOT_FOUND: i8 = 1;
    pub const INTERIOR_MONOLOGUE: i8 = 2;
    pub const NO_GUILD: i8 = 3;
    pub const NO_PARTY: i8 = 4;
    pub const ALLIANCE: i8 = 5;
    pub const INVALID_MAP: i8 = 6;
    pub const NO_PARTY_ARENA: i8 = 7;
    pub const NO_TEAM: i8 = 8;
}

pub mod character_creation_result {
    pub const OK: i8 = 0;
    pub const ERR_NO_REASON: i8 = 1;
    pub const ERR_INVALID_NAME: i8 = 2;
    pub const ERR_NAME_ALREADY_EXISTS: i8 = 3;
    pub const ERR_TOO_MANY_CHARACTERS: i8 = 4;
    pub const ERR_NOT_ALLOWED: i8 = 5;
    pub const ERR_NEW_PLAYER_NOT_ALLOWED: i8 = 6;
    pub const ERR_RESTRICTED_ZONE: i8 = 7;
}
