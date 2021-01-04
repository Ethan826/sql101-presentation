#[macro_use]
extern crate diesel;

mod schema;

use crate::schema::{games, players, players_games, players_teams, teams};
use csv::ReaderBuilder;
use diesel::prelude::*;
use dotenv::dotenv;
use glob::glob;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "players"]
struct Player {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "First")]
    first: String,
    #[serde(rename = "Last")]
    last: String,
}

#[derive(Debug, Deserialize, Insertable, Queryable)]
#[table_name = "teams"]
struct Team {
    id: String,
    league: String,
    city: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Game {
    date: String,
    game_number: String,
    day_of_week: String,
    visiting_team_id: String,
    visiting_team_league: String,
    visiting_team_game_number: String,
    home_team_id: String,
    home_team_league: String,
    home_team_game_number: String,
    day_night: String,
    completion_info: String,
    make_up_date: String,
}

#[derive(Debug, Insertable)]
#[table_name = "games"]
struct NewGame {
    id: String,
    date: String,
    game_number: String,
    day_of_week: String,
    visiting_team_id: String,
    visiting_team_game_number: String,
    home_team_id: String,
    home_team_game_number: String,
    day_night: String,
    completion_info: String,
    make_up_date: String,
}

fn get_reader(
    env_var: &str,
    has_headers: bool,
) -> Result<csv::Reader<std::fs::File>, Box<dyn Error>> {
    let path = env::var(env_var)?;
    let file = File::open(path)?;
    let rdr = ReaderBuilder::new()
        .has_headers(has_headers)
        .from_reader(file);

    Ok(rdr)
}

fn insert_players(conn: &PgConnection) -> Result<(), Box<dyn Error>> {
    let mut rdr = get_reader("PLAYER_FILE", true)?;

    for result in rdr.deserialize() {
        let record: Player = result?;
        diesel::insert_into(players::table)
            .values(&record)
                .on_conflict_do_nothing()
            .execute(conn)?;
    }
    Ok(())
}

fn insert_teams(conn: &PgConnection) -> Result<(), Box<dyn Error>> {
    let mut rdr = get_reader("TEAM_FILE", false)?;

    for result in rdr.deserialize() {
        let record: Team = result?;
        diesel::insert_into(teams::table)
            .values(&record)
            .on_conflict_do_nothing()
            .execute(conn)?;
    }
    Ok(())
}

#[derive(Debug, Insertable, Identifiable)]
#[table_name = "players_games"]
#[primary_key(player_id, game_id)]
struct PlayersGames<'a> {
    player_id: &'a str,
    game_id: &'a str,
}

fn insert_players_games_entry(
    record: &PlayersGames,
    conn: &PgConnection,
) -> Result<(), Box<dyn Error>> {
    println!("Inserting {:?}", &record);

    let result = diesel::insert_into(players_games::table)
        .values(record)
        .on_conflict_do_nothing()
        .execute(conn);

    match result {
        Err(e) => {
            dbg!(e);
            Ok(())
        }
        Ok(_) => Ok(())
    }
}

fn insert_games(conn: &PgConnection) -> Result<(), Box<dyn Error>> {
    let mut rdr = get_reader("SCHEDULE_FILE", false)?;

    for result in rdr.deserialize() {
        let raw: Game = result?;
        let record: NewGame = NewGame {
            id: format!("{}{}{}", raw.home_team_id, raw.date, raw.game_number),
            date: raw.date,
            game_number: raw.game_number,
            day_of_week: raw.day_of_week,
            visiting_team_id: raw.visiting_team_id,
            visiting_team_game_number: raw.visiting_team_game_number,
            home_team_id: raw.home_team_id,
            home_team_game_number: raw.home_team_game_number,
            day_night: raw.day_night,
            completion_info: raw.completion_info,
            make_up_date: raw.make_up_date,
        };

        diesel::insert_into(games::table)
            .values(&record)
        .on_conflict_do_nothing()
            .execute(conn)?;
    }
    Ok(())
}

fn process_line(
    line: &str,
    current_game: &Option<String>,
    conn: &PgConnection,
) -> Result<Option<String>, Box<dyn Error>> {
    if line.starts_with("id,") {
        return Ok(Some(line[3..].to_string()));
    }

    if line.starts_with("start,") {
        let substring = &line[6..];
        let end_index = substring.find(',').expect("Malformed data");
        let player = &substring[0..end_index];

        insert_players_games_entry(
            &PlayersGames {
                player_id: player,
                game_id: current_game.as_ref().unwrap(),
            },
            &conn,
        )?
    } else if line.starts_with("sub,") {
        let substring = &line[4..];
        let end_index = substring.find(',').expect("Malformed data");
        let player = &substring[0..end_index];
        insert_players_games_entry(
            &PlayersGames {
                player_id: player,
                game_id: current_game.as_ref().unwrap(),
            },
            &conn,
        )?
    }

    Ok(None)
}

fn insert_players_games(conn: &PgConnection) -> Result<(), Box<dyn Error>> {
    for f in glob("./**/*.EV*")? {
        let reader = File::open(f?)?;
        let reader = BufReader::new(reader);
        let mut current_game: Option<String> = None;

        for line in reader.lines() {
            if let Some(updated) = process_line(&line?, &current_game, conn)? {
                current_game = Some(updated);
            }
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize, Insertable, Identifiable)]
#[table_name = "players_teams"]
#[primary_key(player_id, team_id)]
struct RosterEntry {
    player_id: String,
    team_id: String,
}

fn insert_players_teams(conn: &PgConnection) -> Result<(), Box<dyn Error>> {
    let teams: Vec<Team> = teams::dsl::teams.load(conn)?;
    for team in teams {
        let reader = File::open(&format!("./retrosheet_files/{}2016.ROS", &team.id))?;
        let reader = BufReader::new(reader);

        for line in reader.lines() {
            let line = line?;
            let team_id = team.id.to_owned();

            let end_index = &line.find(',').expect("Malformed data");
            let record = RosterEntry {
                player_id: line[..*end_index].to_string(),
                team_id,
            };
            println!("Inserting {:?}", &record);

            diesel::insert_into(players_teams::table)
                .values(&record)
                .on_conflict_do_nothing()
                .execute(conn)?;
        }
    }
    Ok(())
}

fn main() {
    dotenv().ok();
    let conn = establish_connection();

    let run = || -> Result<(), Box<dyn Error>> {
        insert_players(&conn)?;
        insert_teams(&conn)?;
        insert_games(&conn)?;
        insert_players_games(&conn)?;
        insert_players_teams(&conn)?;
        Ok(())
    };

    if let Err(e) = run() {
        dbg!(e);
    }
}
