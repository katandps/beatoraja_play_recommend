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
use std::collections::HashMap;
use crate::song_data::SongData;
use crate::song::{HashSha256, HashMd5};

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

    let database_url = env::var("SCORE_DATABASE_URL").expect("SCORE_DATABASE_URL must be set");
    let connection = establish_connection(database_url);
    let results = score
        .load::<crate::model::score::Score>(&connection)
        .expect("Error loading schema");

    make_whole_score(results)
}

fn make_whole_score(record: Vec<crate::model::score::Score>) -> WholeScore {
    let mut scores = HashMap::new();
    for row in record {
        let song_id = SongId::new(HashSha256::new(row.sha256), row.mode);
        let clear = ClearType::from_integer(row.clear);
        let updated_at = UpdatedAt::new(DateTime::from(Local.timestamp(row.date as i64, 0)));
        scores.insert(song_id, Score::from_data(clear, updated_at));
    }
    WholeScore::new(Scores::new(scores))
}

pub fn establish_connection(url: String) -> SqliteConnection {
    SqliteConnection::establish(&url)
        .unwrap_or_else(|_| panic!("Error connection to {}", &url))
}

pub fn song_data() -> SongData {
    use super::schema::song::song::dsl::*;
    dotenv().ok();

    let database_url = env::var("SONG_DATABASE_URL").expect("SONG_DATABASE_URL must be set");
    let connection = establish_connection(database_url);
    let results = song
        .load::<crate::model::song::Song>(&connection)
        .expect("Error loading schema");
    make_song_data(results)
}

fn make_song_data(record: Vec<crate::model::song::Song>) -> SongData {
    let mut md5_to_sha256 = HashMap::new();
    let mut sha256_to_md5 = HashMap::new();
    for row in record {
        md5_to_sha256.insert(HashMd5::new(row.md5.clone()), HashSha256::new(row.sha256.clone()));
        sha256_to_md5.insert(HashSha256::new(row.sha256), HashMd5::new(row.md5));
    }
    SongData::new(md5_to_sha256, sha256_to_md5)
}