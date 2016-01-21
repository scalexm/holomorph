use diesel::*;
use diesel::types::Text;

sql_function!(lower, lower_t, (x: Text) -> Text);

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
        channels -> Array<SmallInt>,
        max_characters_count -> SmallInt,
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

table! {
    maps {
        id -> Serial,
        left -> Integer,
        right -> Integer,
        top -> Integer,
        bottom -> Integer,
        client_top -> Integer,
        client_bottom -> Integer,
        custom_left_cell -> SmallInt,
        custom_right_cell -> SmallInt,
        custom_top_cell -> SmallInt,
        custom_bottom_cell -> SmallInt,
        cells -> Binary,
    }
}

table! {
    map_positions {
        id -> Serial,
        pos_x -> SmallInt,
        pos_y -> SmallInt,
        outdoor -> Bool,
        capabilities -> Integer,
        sub_area_id -> SmallInt,
    }
}

select_column_workaround!(map_positions -> maps (
    id,
    pos_x,
    pos_y,
    outdoor,
    capabilities,
    sub_area_id
));
select_column_workaround!(maps -> map_positions (
    id,
    left,
    right,
    top, bottom,
    client_top,
    client_bottom,
    custom_left_cell,
    custom_right_cell,
    custom_top_cell,
    custom_bottom_cell, cells
));
joinable!(maps -> map_positions (id = id));

table! {
    sub_areas {
        id -> SmallSerial,
        area_id -> SmallInt,
        monsters -> Array<SmallInt>,
    }
}

table! {
    areas {
        id -> SmallSerial,
        priority -> SmallInt,
    }
}

table! {
    character_minimals {
        id -> BigSerial,
        account_id -> Integer,
        account_nickname -> Text,
        level -> SmallInt,
        name -> Text,
        breed -> SmallInt,
        sex -> Bool,
        look -> Binary,
        mood_smiley -> SmallInt,
    }
}

table! {
    characters {
        id -> Nullable<BigSerial>,
        xp -> BigInt,
        kamas -> Integer,
        stats_points -> SmallInt,
        additionnal_points -> SmallInt,
        spells_points -> SmallInt,
        energy_points -> SmallInt,

        base_vitality -> SmallInt,
        base_wisdom -> SmallInt,
        base_strength -> SmallInt,
        base_intelligence -> SmallInt,
        base_chance -> SmallInt,
        base_agility -> SmallInt,

        additionnal_vitality -> SmallInt,
        additionnal_wisdom -> SmallInt,
        additionnal_strength -> SmallInt,
        additionnal_intelligence -> SmallInt,
        additionnal_chance -> SmallInt,
        additionnal_agility -> SmallInt,

        map_id -> Integer,
        cell_id -> SmallInt,
        direction -> SmallInt,
    }
}

table! {
    connections_history {
        id -> Serial,
        account_id -> Integer,
        date -> BigInt,
        ip -> Text,
    }
}

table! {
    social_relations {
        id -> Serial,
        warn_on_connection -> Bool,
        warn_on_level_gain -> Bool,
        friends -> Array<Integer>,
        ignored -> Array<Integer>,
    }
}

table! {
    breeds {
        id -> SmallSerial,
        male_look -> Binary,
        female_look -> Binary,
        spawn_map -> Integer,
    }
}

table! {
    breed_heads {
        id -> SmallSerial,
        breed_id -> SmallInt,
        skin -> SmallInt,
        gender -> Bool,
    }
}
