extern crate diesel;
extern crate dotenv;

use std::env;

use dotenv::dotenv;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::model::player::Player;
use crate::score::{WholeScore, Scores, Score, SongId, ClearType};

pub fn run()
{
    use super::schema::player::player::dsl::*;
    dotenv().ok();

    let database_url = env::var("SCORE_DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = establish_connection(database_url);
    let results: Vec<Player> = player
        .limit(5)
        .load::<Player>(&connection)
        .expect("Error loading schema");

    println!("Displaying {} schema", results.len());
    for result in results {
        println!("{}", result.playcount);
        println!("{}", result.clear);
        println!(" ")
    }
}

pub fn score() -> WholeScore {
    use super::schema::score::score::dsl::*;
    dotenv().ok();

    let database_url = env::var("SCORE_DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = establish_connection(database_url);
    let results = score
        .load::<crate::model::score::Score>(&connection)
        .expect("Error loading schema");

    WholeScore::new(
        Scores::new(
            results
                .iter()
                .map(
                    |s|
                        Score::new(
                            SongId::new((&s.sha256).parse().unwrap(), s.mode),
                            ClearType::from_integer(s.clear),
                        )
                ).collect()
        )
    )
}

pub fn establish_connection(url: String) -> SqliteConnection {
    SqliteConnection::establish(&url)
        .unwrap_or_else(|_| panic!("Error connection to {}", &url))
}
