table! {
    races (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

table! {
    riders (id) {
        id -> Nullable<Integer>,
        given_name -> Text,
        surname -> Text,
    }
}

table! {
    riders_races (rider_id, year, team_id) {
        rider_id -> Integer,
        race_id -> Integer,
        year -> Integer,
        team_id -> Integer,
        finishing_place -> Nullable<Integer>,
        result -> Nullable<Text>,
    }
}

table! {
    riders_teams (rider_id, team_id, year) {
        rider_id -> Nullable<Integer>,
        team_id -> Nullable<Integer>,
        year -> Nullable<Integer>,
    }
}

table! {
    teams (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

joinable!(riders_races -> races (race_id));
joinable!(riders_races -> riders (rider_id));
joinable!(riders_races -> teams (team_id));
joinable!(riders_teams -> riders (rider_id));
joinable!(riders_teams -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    races,
    riders,
    riders_races,
    riders_teams,
    teams,
);
