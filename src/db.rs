use diesel;
use dotenv;

use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::schema::player::Player;
use crate::score::scores::Scores;
use crate::score::song_id::{PlayMode, SongId};
use crate::score::Score;
use crate::score_log;
use crate::score_log::SnapShot;
use crate::song::hash::{HashMd5, HashSha256};
use crate::song::hash_converter;
use std::collections::HashMap;

fn establish_connection(env_key: &str) -> SqliteConnection {
    dotenv::dotenv().ok();
    let url = env::var(env_key).expect(format!("{} must be set", env_key).as_ref());
    SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Error connection to {}", &url))
}

pub fn player() {
    use super::schema::player::player::dsl::*;
    let connection = establish_connection("SCORE_DATABASE_URL");
    let results: Vec<Player> = player
        .load::<Player>(&connection)
        .expect("Error loading schema");

    let last = results.last().unwrap();
    println!("PlayCount: {}", last.playcount);
    println!("ClearCount: {}", last.clear);
    println!("PlayTime: {}", last.playtime);
    println!()
}

pub fn score() -> Scores {
    use super::schema::score::score::dsl::*;
    let connection = establish_connection("SCORE_DATABASE_URL");
    let results = score
        .load::<crate::schema::score::Score>(&connection)
        .expect("Error loading schema");

    make_whole_score(results)
}

fn make_whole_score(record: Vec<crate::schema::score::Score>) -> Scores {
    let mut scores = HashMap::new();
    for row in record {
        scores.insert(
            SongId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode)),
            Score::from_data(
                row.clear,
                row.date,
                row.epg,
                row.lpg,
                row.egr,
                row.lgr,
                row.egd,
                row.lgd,
                row.ebd,
                row.lbd,
                row.epr,
                row.lpr,
                row.ems,
                row.lms,
                row.combo,
                row.playcount,
                row.minbp,
            ),
        );
    }
    Scores::new(scores)
}

pub fn song_data() -> hash_converter::Converter {
    use super::schema::song::song::dsl::*;
    let connection = establish_connection("SONG_DATABASE_URL");
    let results = song
        .load::<crate::schema::song::Song>(&connection)
        .expect("Error loading schema");
    make_song_data(results)
}

fn make_song_data(record: Vec<crate::schema::song::Song>) -> hash_converter::Converter {
    let mut builder = hash_converter::Builder::new();
    for row in record {
        builder.push(HashMd5::new(row.md5), HashSha256::new(row.sha256));
    }
    hash_converter::Builder::build(builder)
}

pub fn score_log() -> score_log::ScoreLog {
    use crate::schema::score_log::scorelog::dsl::*;
    let connection = establish_connection("SCORELOG_DATABASE_URL");
    let results = scorelog
        .load::<crate::schema::score_log::ScoreLog>(&connection)
        .expect("Error loading schema");
    make_score_log(results)
}

fn make_score_log(record: Vec<crate::schema::score_log::ScoreLog>) -> score_log::ScoreLog {
    let mut builder = score_log::Builder::builder();
    for row in record {
        let song_id = SongId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode));
        let snapshot = SnapShot::from_data(
            song_id.clone(),
            row.clear,
            row.score,
            row.combo,
            row.minbp,
            row.date,
        );
        builder.push(song_id, snapshot)
    }
    score_log::Builder::build(builder)
}
