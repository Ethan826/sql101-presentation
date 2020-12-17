CREATE TABLE riders_races (
  rider_id INTEGER NOT NULL,
  race_id INTEGER NOT NULL,
  year INTEGER NOT NULL,

  team_id INTEGER NOT NULL,
  finishing_place INTEGER,
  result TEXT,

  PRIMARY KEY (rider_id, team_id, year),

  FOREIGN KEY (rider_id) REFERENCES riders(id) ON DELETE CASCADE,
  FOREIGN KEY (race_id) REFERENCES races(id) ON DELETE CASCADE,
  FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE
);
