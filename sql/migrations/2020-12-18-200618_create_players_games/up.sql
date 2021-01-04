CREATE TABLE players_games (
  player_id VARCHAR(8) NOT NULL,
  game_id VARCHAR(12) NOT NULL,

  PRIMARY KEY (player_id, game_id),

  FOREIGN KEY (player_id) REFERENCES players(id),
  FOREIGN KEY (game_id) REFERENCES games(id),

  CONSTRAINT unique_players_games UNIQUE (player_id, game_id)
);
