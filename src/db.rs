extern crate diesel;
extern crate dotenv;

use std::env;

use dotenv::dotenv;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::model::player::Player;
use crate::whole_score::WholeScore;
use crate::whole_score::scores::Scores;
use crate::whole_score::scores::score::Score;
use crate::whole_score::scores::score::song_id::SongId;
use crate::whole_score::scores::score::clear_type::ClearType;
use crate::whole_score::scores::score::updated_at::UpdatedAt;
use chrono::{DateTime, Local, TimeZone};

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

    WholeScore::new(Scores::new(results.iter().map(make_score()).collect()))
}

fn make_score() -> Box<dyn FnMut(&crate::model::score::Score) -> Score> {
    Box::new(|score| {
        Score::new(
            SongId::new((score.sha256).parse().unwrap(), score.mode),
            ClearType::from_integer(score.clear),
            UpdatedAt::new(
                DateTime::from(Local.timestamp(score.date as i64, 0)
                )
            ),
        )
    })
}

pub fn establish_connection(url: String) -> SqliteConnection {
    SqliteConnection::establish(&url)
        .unwrap_or_else(|_| panic!("Error connection to {}", &url))
}
