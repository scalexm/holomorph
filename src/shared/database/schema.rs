table! {
    logs {
        id -> Serial,
        account_id -> Integer,
        date -> BigInt,
        log_type -> Text,
        content -> Text,
    }
}

table! {
    accounts {
        id -> Serial,
        account -> Text,
        password -> Text,
        salt -> Text,
        nickname -> Text,
        secret_question -> Text,
        secret_answer -> Text,
        level -> SmallInt,
        subscription_end -> BigInt,
        creation_date -> BigInt,
        already_logged -> SmallInt,
        ban_end -> BigInt,
        ticket -> Text,
        last_server -> SmallInt,
    }
}

table! {
    ip_bans {
        id -> Serial,
        ip -> Text,
    }
}

table! {
    character_counts {
        id -> Serial,
        account_id -> Integer,
        server_id -> SmallInt,
    }
}

table! {
    game_servers {
        id -> SmallSerial,
        key -> Text,
        min_level -> SmallInt,
    }
}
