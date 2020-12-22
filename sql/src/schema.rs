table! {
    games (id) {
        id -> Text,
        date -> Text,
        game_number -> Text,
        day_of_week -> Text,
        visiting_team_id -> Text,
        visiting_team_game_number -> Text,
        home_team_id -> Text,
        home_team_game_number -> Text,
        day_night -> Text,
        completion_info -> Nullable<Text>,
        make_up_date -> Nullable<Text>,
    }
}

table! {
    players (id) {
        id -> Text,
        first -> Text,
        last -> Text,
    }
}

table! {
    players_games (player_id, game_id) {
        player_id -> Text,
        game_id -> Text,
    }
}

table! {
    players_teams (player_id, team_id) {
        player_id -> Text,
        team_id -> Text,
    }
}

table! {
    teams (id) {
        id -> Text,
        league -> Text,
        city -> Text,
        name -> Text,
    }
}

joinable!(players_games -> players (player_id));
joinable!(players_teams -> players (player_id));
joinable!(players_teams -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    games,
    players,
    players_games,
    players_teams,
    teams,
);
