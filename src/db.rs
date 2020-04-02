use diesel;
use dotenv;

use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::model::player::Player;
use crate::score::clear_type::ClearType;
use crate::score::judge::Judge;
use crate::score::max_combo::MaxCombo;
use crate::score::min_bp::MinBP;
use crate::score::play_count::PlayCount;
use crate::score::scores::Scores;
use crate::score::song_id::{PlayMode, SongId};
use crate::score::updated_at::UpdatedAt;
use crate::score::Score;
use crate::song::{HashMd5, HashSha256};
use crate::song_data::{Builder, SongData};
use chrono::{DateTime, Local, TimeZone};
use std::cmp::min;
use std::collections::HashMap;

pub fn run() {
    use super::schema::player::player::dsl::*;
    dotenv::dotenv().ok();

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

pub fn score() -> Scores {
    use super::schema::score::score::dsl::*;
    dotenv::dotenv().ok();

    let database_url = env::var("SCORE_DATABASE_URL").expect("SCORE_DATABASE_URL must be set");
    let connection = establish_connection(database_url);
    let results = score
        .load::<crate::model::score::Score>(&connection)
        .expect("Error loading schema");

    make_whole_score(results)
}

fn make_whole_score(record: Vec<crate::model::score::Score>) -> Scores {
    let mut scores = HashMap::new();
    for row in record {
        let song_id = SongId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode));
        let clear = ClearType::from_integer(row.clear);
        let updated_at = UpdatedAt::new(DateTime::from(Local.timestamp(row.date as i64, 0)));
        let judge = Judge::new(
            row.epg, row.lpg, row.egr, row.lgr, row.egd, row.lgd, row.ebd, row.lbd, row.epr,
            row.lpr, row.ems, row.lms,
        );
        let max_combo = MaxCombo::new(row.combo);
        let play_count = PlayCount::new(row.playcount);
        let min_bp = MinBP::new(row.minbp);
        scores.insert(
            song_id,
            Score::from_data(clear, updated_at, judge, max_combo, play_count, min_bp),
        );
    }
    Scores::new(scores)
}

pub fn establish_connection(url: String) -> SqliteConnection {
    SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Error connection to {}", &url))
}

pub fn song_data() -> SongData {
    use super::schema::song::song::dsl::*;
    dotenv::dotenv().ok();

    let database_url = env::var("SONG_DATABASE_URL").expect("SONG_DATABASE_URL must be set");
    let connection = establish_connection(database_url);
    let results = song
        .load::<crate::model::song::Song>(&connection)
        .expect("Error loading schema");
    make_song_data(results)
}

fn make_song_data(record: Vec<crate::model::song::Song>) -> SongData {
    let mut builder = Builder::new();
    for row in record {
        builder.push(HashMd5::new(row.md5), HashSha256::new(row.sha256));
    }
    Builder::build(builder)
}
