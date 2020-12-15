mod schema;

use diesel::prelude::*;
use model::*;
use std::collections::HashMap;

#[macro_use]
extern crate diesel;
extern crate anyhow;

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

    fn score_log(&self) -> HashMap<SongId, SnapShots> {
        use schema::score_log::scorelog::dsl::*;
        let connection = Self::establish_connection(&self.scorelog_db_url);
        let record: Vec<schema::score_log::ScoreLog> =
            scorelog.load(&connection).expect("Error loading schema");

        let mut map = HashMap::new();
        for row in record {
            let song_id = SongId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode));
            let snap = SnapShot::from_data(row.clear, row.score, row.combo, row.minbp, row.date);
            map.entry(song_id).or_insert(SnapShots::default()).add(snap);
        }
        map
    }

    pub fn player(&self) -> PlayerStates {
        use schema::player::player::dsl::*;
        let connection = Self::establish_connection(&self.score_db_url);
        let records: Vec<schema::player::Player> = player
            .load::<schema::player::Player>(&connection)
            .expect("Error loading schema");

        let mut log = Vec::new();
        for row in records {
            let pl = PlayerState::new(
                PlayCount::new(row.playcount),
                PlayCount::new(row.clear),
                PlayTime::new(row.playtime),
                UpdatedAt::from_timestamp(row.date),
                TotalJudge::new(Judge::new(
                    row.epg, row.lpg, row.egr, row.lgr, row.egd, row.lgd, row.ebd, row.lbd,
                    row.epr, row.lpr, row.ems, row.lms,
                )),
            );
            log.push(pl);
        }
        PlayerStates::new(log)
    }
}

impl ScoreRepository for SqliteClient {
    fn score(&self) -> Scores {
        use schema::score::score::dsl::*;
        let connection = Self::establish_connection(&self.score_db_url);
        let record = score
            .load::<schema::score::Score>(&connection)
            .expect("Error loading schema");
        let score_log = self.score_log();
        Scores::new(
            record
                .iter()
                .map(|row| {
                    let song_id = SongId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode));
                    (
                        song_id.clone(),
                        Score::new(
                            ClearType::from_integer(row.clear),
                            UpdatedAt::from_timestamp(row.date),
                            Judge::new(
                                row.epg, row.lpg, row.egr, row.lgr, row.egd, row.lgd, row.ebd,
                                row.lbd, row.epr, row.lpr, row.ems, row.lms,
                            ),
                            MaxCombo::from_combo(row.combo),
                            PlayCount::new(row.playcount),
                            MinBP::from_bp(row.minbp),
                            score_log.get(&song_id).unwrap().clone(),
                        ),
                    )
                })
                .collect::<HashMap<SongId, Score>>(),
        )
    }
}

impl SongRepository for SqliteClient {
    fn song_data(&self) -> Songs {
        use schema::song::song::dsl::*;
        let connection = Self::establish_connection(&self.song_db_url);
        let record: Vec<schema::song::Song> = song.load(&connection).expect("Error loading schema");

        record
            .iter()
            .fold(SongsBuilder::new(), |mut builder, row| {
                builder.push(
                    HashMd5::new(row.md5.clone()),
                    HashSha256::new(row.sha256.clone()),
                    Title::new(format!("{}{}", row.title, row.subtitle)),
                    Artist::new(row.artist.clone()),
                    row.notes,
                );
                builder
            })
            .build()
    }

    fn save_song(&self, _songs: &Songs) {
        unimplemented!()
    }
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
