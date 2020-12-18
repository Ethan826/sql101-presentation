table! {
    players (id) {
        id -> Nullable<Text>,
        first -> Text,
        last -> Text,
    }
}

table! {
    teams (id) {
        id -> Nullable<Text>,
        league -> Text,
        city -> Text,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    players,
    teams,
);
