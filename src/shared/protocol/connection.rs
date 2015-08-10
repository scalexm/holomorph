use std::io::{Read, Write};
use io::Result;
use super::*;

impl_type!(HelloConnectMessage, 3,
    salt| String,
    key| VarIntVec<u8>);

impl_type!(ClearIdentificationMessage, 888,
    username| String,
    password| String);

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
