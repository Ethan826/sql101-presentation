CREATE TABLE players_teams (
  player_id VARCHAR(8) NOT NULL,
  team_id VARCHAR(3) NOT NULL,

  PRIMARY KEY (player_id, team_id),

  FOREIGN KEY (player_id) REFERENCES players(id),
  FOREIGN KEY (team_id) REFERENCES teams(id),

  CONSTRAINT unique_players_teams UNIQUE (player_id, team_id)
);
