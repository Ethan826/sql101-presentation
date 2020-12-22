CREATE TABLE players_games (
  player_id VARCHAR(8) NOT NULL,
  game_id VARCHAR(7) NOT NULL,

  PRIMARY KEY (player_id, game_id),

  FOREIGN KEY (player_id) REFERENCES players(id),
  FOREIGN KEY (game_id) REFERENCES game(id)
);
