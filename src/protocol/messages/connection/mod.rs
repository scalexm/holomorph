pub mod search;
pub mod register;
use std::io::{Read, Write};
use std::io::Result;
use protocol::*;
 use types::connection::GameServerInformations; use types::version::VersionExtended; use types::version::Version;
impl_type!(CredentialsAcknowledgementMessage, 6314);
impl_type!(HelloConnectMessage, 3, salt| String, key| VarIntVec<u8>);
impl_type!(IdentificationAccountForceMessage, 6119, base| IdentificationMessage, forced_account_login| String);
impl_type!(IdentificationFailedBannedMessage, 6174, base| IdentificationFailedMessage, ban_end_date| f64);
impl_type!(IdentificationFailedForBadVersionMessage, 21, base| IdentificationFailedMessage, required_version| Version);
impl_type!(IdentificationFailedMessage, 20, reason| i8);
impl_type!(IdentificationMessage, 4, autoconnect| Flag, use_certificate| Flag, use_login_token| Flag, version| VersionExtended, lang| String, credentials| VarIntVec<u8>, server_id| i16, session_optional_salt| VarLong, failed_attempts| Vec<VarShort>);
impl_type!(IdentificationSuccessMessage, 22, has_rights| Flag, was_already_connected| Flag, login| String, nickname| String, account_id| i32, community_id| i8, secret_question| String, account_creation| f64, subscription_elapsed_duration| f64, subscription_end_date| f64);
impl_type!(IdentificationSuccessWithLoginTokenMessage, 6209, base| IdentificationSuccessMessage, login_token| String);
impl_type!(SelectedServerDataExtendedMessage, 6469, base| SelectedServerDataMessage, server_ids| Vec<VarShort>);
impl_type!(SelectedServerDataMessage, 42, server_id| VarShort, address| String, port| i16, can_create_new_character| bool, ticket| VarIntVec<u8>);
impl_type!(SelectedServerRefusedMessage, 41, server_id| VarShort, error| i8, server_status| i8);
impl_type!(ServerSelectionMessage, 40, server_id| VarShort);
impl_type!(ServersListMessage, 30, servers| Vec<GameServerInformations>, already_connected_to_server_id| VarShort, can_create_new_character| bool);
impl_type!(ServerStatusUpdateMessage, 50, server| GameServerInformations);