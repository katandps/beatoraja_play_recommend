mod schema;

use diesel::prelude::*;
use model::*;
use std::collections::HashMap;

#[macro_use]
extern crate diesel;
extern crate anyhow;

fn establish_connection(url: &str) -> SqliteConnection {
    SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Error connection to {}", &url))
}

pub fn player() {
    use schema::player::player::dsl::*;
    let connection = establish_connection(&config().score_db_url());
    let results: Vec<schema::player::Player> = player
        .load::<schema::player::Player>(&connection)
        .expect("Error loading schema");

    let last = results.last().unwrap();
    println!("\nPlayCount: {}", last.playcount);
    println!("ClearCount: {}", last.clear);
    println!("PlayTime: {}", last.playtime);
    println!()
}

// 現在のスコアを詳細に出力する機能はいまのところない
#[allow(dead_code)]
pub fn score() -> Scores {
    use schema::score::score::dsl::*;
    let connection = establish_connection(&config().score_db_url());
    let results = score
        .load::<schema::score::Score>(&connection)
        .expect("Error loading schema");

    make_whole_score(results)
}
#[allow(dead_code)]
fn make_whole_score(record: Vec<schema::score::Score>) -> Scores {
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

pub fn song_data() -> Songs {
    use schema::song::song::dsl::*;
    let connection = establish_connection(&config().song_db_url());
    let results = song
        .load::<schema::song::Song>(&connection)
        .expect("Error loading schema");
    make_song_data(results)
}

fn make_song_data(record: Vec<schema::song::Song>) -> Songs {
    let mut builder = SongsBuilder::new();
    for row in record {
        builder.push(
            HashMd5::new(row.md5),
            HashSha256::new(row.sha256),
            Title::make(row.title),
            Artist::make(row.artist),
            row.notes,
        );
    }
    SongsBuilder::build(builder)
}

pub fn score_log() -> ScoreLog {
    use schema::score_log::scorelog::dsl::*;
    let connection = establish_connection(&config().scorelog_db_url());
    let results = scorelog
        .load::<schema::score_log::ScoreLog>(&connection)
        .expect("Error loading schema");
    make_score_log(results)
}

fn make_score_log(record: Vec<schema::score_log::ScoreLog>) -> ScoreLog {
    let mut builder = ScoreLogBuilder::builder();
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
    ScoreLogBuilder::build(builder)
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
