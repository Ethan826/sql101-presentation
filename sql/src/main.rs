#[macro_use]
extern crate diesel;

mod schema;

use crate::schema::players;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::File;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
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

fn get_reader(env_var: &str) -> Result<csv::Reader<std::fs::File>, Box<dyn Error>> {
    let path = env::var(env_var)?;
    let file = File::open(path)?;

    Ok(csv::Reader::from_reader(file))
}

fn insert_players(conn: &SqliteConnection) -> Result<(), Box<dyn Error>> {
    let mut rdr = get_reader("PLAYER_FILE")?;

    for result in rdr.deserialize() {
        let record: Player = result?;
        diesel::insert_into(players::table)
            .values(&record)
            .execute(conn)
            .unwrap_or_else(|_| panic!("Error inserting {:?}", record));
    }
    Ok(())
}

fn main() {
    dotenv().ok();
    let conn = establish_connection();

    insert_players(&conn).ok();
}
