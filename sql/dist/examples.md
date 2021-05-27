- What player has the id `'dillp101'`?

```sql
SELECT * FROM players WHERE id = 'dillp101';
```

- How many teams were there in 2016?



```sql
SELECT COUNT(*) FROM teams;
```

- How many players played in 2016?

```sql
SELECT COUNT(DISTINCT player_id) FROM players_teams;
```

- How many games did Anthony Rizzo play in 2016?

```sql
SELECT COUNT(pg.game_id)
FROM players player
INNER JOIN players_games pg
        ON player.id = pg.player_id
WHERE first = 'Anthony'
  AND last = 'Rizzo';
```

- What is the commonest first name in MLB history?

```sql
SELECT COUNT(first), first
FROM players
GROUP BY first
ORDER BY 1 DESC
LIMIT 1;
```

- List all players in the database and include all their 2016 teams.

```sql
SELECT p.first, p.last, t.name
FROM players_teams pt
INNER JOIN teams t
        ON pt.team_id = t.id
RIGHT OUTER JOIN players p
              ON pt.player_id = p.id;
```

- What player played for the most teams (choose only one if it's a tie), and what teams were they?


```sql
SELECT sub.first AS "First Name",
       sub.last AS "Last Name",
       team.name AS "Team Name"
FROM (
  SELECT player.id,
         player.first,
         player.last,
         COUNT(pt.team_id) AS tally
  FROM players player
  INNER JOIN players_teams pt
          ON pt.player_id = player.id
  GROUP BY player.id
  ORDER BY tally DESC
  LIMIT 1
) sub
INNER JOIN players_teams pt
        ON pt.player_id = sub.id
INNER JOIN teams team
        ON team.id = pt.team_id;
```
- From those players who played at least 10 total games in 2016, what player played the highest percentage of night games?

```sql
SELECT player.first,
       player.last,
       CAST(night_games.tally AS FLOAT) / total_games.tally AS "Percent"
FROM ( SELECT pg.player_id, COUNT(pg.player_id) AS tally
       FROM players_games pg
       INNER JOIN games game ON game.id = pg.game_id
       WHERE game.day_night = 'N'
       GROUP BY pg.player_id
) AS night_games INNER JOIN (
       SELECT pg.player_id, COUNT(pg.player_id) AS tally
       FROM players_games pg
       INNER JOIN games game ON game.id = pg.game_id
       GROUP BY pg.player_id
) AS total_games ON night_games.player_id = total_games.player_id
INNER JOIN players player ON player.id = total_games.player_id
WHERE total_games.tally >= 10
ORDER BY "Percent" DESC
LIMIT 1;
```
