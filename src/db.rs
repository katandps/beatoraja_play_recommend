use diesel;
use dotenv;

use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::schema::player::Player;
use crate::score::clear_type::ClearType;
use crate::score::ex_score::ExScore;
use crate::score::judge::Judge;
use crate::score::max_combo::MaxCombo;
use crate::score::min_bp::MinBP;
use crate::score::play_count::PlayCount;
use crate::score::scores::Scores;
use crate::score::song_id::{PlayMode, SongId};
use crate::score::updated_at::UpdatedAt;
use crate::score::Score;
use crate::score_log;
use crate::score_log::SnapShot;
use crate::song::{HashMd5, HashSha256};
use crate::song_data;
use std::collections::HashMap;

fn establish_connection(env_key: &str) -> SqliteConnection {
    dotenv::dotenv().ok();
    let url = env::var(env_key).expect(format!("{} must be set", env_key).as_ref());
    SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Error connection to {}", &url))
}

pub fn run() {
    use super::schema::player::player::dsl::*;
    let connection = establish_connection("SCORE_DATABASE_URL");
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
                ClearType::from_integer(row.clear),
                UpdatedAt::from_timestamp(row.date),
                Judge::new(
                    row.epg, row.lpg, row.egr, row.lgr, row.egd, row.lgd, row.ebd, row.lbd,
                    row.epr, row.lpr, row.ems, row.lms,
                ),
                MaxCombo::new(row.combo),
                PlayCount::new(row.playcount),
                MinBP::new(row.minbp),
            ),
        );
    }
    Scores::new(scores)
}

pub fn song_data() -> song_data::SongData {
    use super::schema::song::song::dsl::*;
    let connection = establish_connection("SONG_DATABASE_URL");
    let results = song
        .load::<crate::schema::song::Song>(&connection)
        .expect("Error loading schema");
    make_song_data(results)
}

fn make_song_data(record: Vec<crate::schema::song::Song>) -> song_data::SongData {
    let mut builder = song_data::Builder::new();
    for row in record {
        builder.push(HashMd5::new(row.md5), HashSha256::new(row.sha256));
    }
    song_data::Builder::build(builder)
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
    let mut builder = score_log::Builder::new();
    for row in record {
        let song_id = SongId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode));
        let snapshot = SnapShot::new(
            song_id.clone(),
            ClearType::from_integer(row.clear),
            ExScore::make_by_score(row.score),
            MaxCombo::new(row.combo),
            MinBP::new(row.minbp),
            UpdatedAt::from_timestamp(row.date),
        );
        builder.push(song_id, snapshot)
    }
    score_log::Builder::build(builder)
}
