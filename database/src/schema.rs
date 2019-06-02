table! {
    accounts (id) {
        id -> Int4,
        login -> Varchar,
        password -> Text,
        nickname -> Varchar,
        role -> Int2,
        ticket -> Nullable<Varchar>,
        last_server -> Nullable<Int2>,
    }
}

table! {
    characters (id) {
        id -> Int4,
        account_id -> Int4,
        server_id -> Int2,
    }
}

table! {
    game_servers (id) {
        id -> Int2,
        host -> Text,
        port -> Int2,
    }
}

joinable!(characters -> accounts (account_id));
joinable!(characters -> game_servers (server_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    characters,
    game_servers,
);
