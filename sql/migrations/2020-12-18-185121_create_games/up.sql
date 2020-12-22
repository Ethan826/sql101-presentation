CREATE TABLE games (
    id VARCHAR(7) PRIMARY KEY NOT NULL,
    date TEXT NOT NULL,
    game_number TEXT NOT NULL,
    day_of_week TEXT NOT NULL,
    visiting_team_id TEXT NOT NULL,
    visiting_team_game_number TEXT NOT NULL,
    home_team_id TEXT NOT NULL,
    home_team_game_number TEXT NOT NULL,
    day_night VARCHAR(1) NOT NULL,
    completion_info TEXT,
    make_up_date TEXT,

    FOREIGN KEY (visiting_team_id) REFERENCES teams(id),
    FOREIGN KEY (home_team_id) REFERENCES teams(id)
);
