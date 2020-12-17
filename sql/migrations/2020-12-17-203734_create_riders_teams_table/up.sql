CREATE TABLE riders_teams (
  rider_id INTEGER,
  team_id INTEGER,
  year INTEGER,

  PRIMARY KEY (rider_id, team_id, year),

  FOREIGN KEY (rider_id) REFERENCES riders(id) ON DELETE CASCADE,
  FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE
);
