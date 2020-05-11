mod schema;

use diesel::prelude::*;
use model::*;
use std::collections::HashMap;

#[macro_use]
extern crate diesel;
extern crate anyhow;

pub fn player() {
    use schema::player::player::dsl::*;
    fn establish_connection(url: &str) -> SqliteConnection {
        SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Error connection to {}", &url))
    }

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

pub struct SqliteClient {
    scorelog_db_url: String,
    song_db_url: String,
    score_db_url: String,
}

impl SqliteClient {
    pub fn new() -> SqliteClient {
        SqliteClient {
            scorelog_db_url: config().scorelog_db_url(),
            song_db_url: config().song_db_url(),
            score_db_url: config().score_db_url(),
        }
    }

    fn establish_connection(url: &str) -> SqliteConnection {
        SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Error connection to {}", &url))
    }
}

impl ScoreRepository for SqliteClient {
    fn score(&self) -> Scores {
        use schema::score::score::dsl::*;
        let connection = Self::establish_connection(&self.score_db_url);
        let record = score
            .load::<schema::score::Score>(&connection)
            .expect("Error loading schema");

        let mut scores = HashMap::new();
        for row in record {
            scores.insert(
                SongId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode)),
                Score::new(
                    ClearType::from_integer(row.clear),
                    UpdatedAt::from_timestamp(row.date),
                    Judge::new(
                        row.epg, row.lpg, row.egr, row.lgr, row.egd, row.lgd, row.ebd, row.lbd,
                        row.epr, row.lpr, row.ems, row.lms,
                    ),
                    MaxCombo::from_combo(row.combo),
                    PlayCount::new(row.playcount),
                    MinBP::from_bp(row.minbp),
                ),
            );
        }
        Scores::new(scores)
    }
}

impl SongRepository for SqliteClient {
    fn song_data(&self) -> Songs {
        use schema::song::song::dsl::*;
        let connection = Self::establish_connection(&self.song_db_url);
        let record = song
            .load::<schema::song::Song>(&connection)
            .expect("Error loading schema");

        SongsBuilder::build(record.iter().fold(SongsBuilder::new(), |mut builder, row| {
            builder.push(
                HashMd5::new(row.md5.clone()),
                HashSha256::new(row.sha256.clone()),
                Title::new(row.title.clone()),
                Artist::new(row.artist.clone()),
                row.notes,
            );
            builder
        }))
    }
}

impl ScoreLogRepository for SqliteClient {
    fn score_log(&self) -> ScoreLog {
        use schema::score_log::scorelog::dsl::*;
        let connection = Self::establish_connection(&self.scorelog_db_url);
        let record = scorelog
            .load::<schema::score_log::ScoreLog>(&connection)
            .expect("Error loading schema");

        ScoreLogBuilder::build(record.iter().fold(
            ScoreLogBuilder::builder(),
            |mut builder, row| {
                builder.push(
                    SongId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode)),
                    SnapShot::from_data(row.clear, row.score, row.combo, row.minbp, row.date),
                );
                builder
            },
        ))
    }
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
