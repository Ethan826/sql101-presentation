CREATE TABLE riders (
  id INTEGER PRIMARY KEY,
  nation_id VARCHAR(3) NOT NULL,
  given_name TEXT NOT NULL,
  surname TEXT NOT NULL,

  FOREIGN KEY (nation_id) REFERENCES nations(id)
);
