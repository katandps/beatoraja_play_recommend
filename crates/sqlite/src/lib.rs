mod schema;

use anyhow::Result;
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
    pub fn new(scorelog_db_url: String, song_db_url: String, score_db_url: String) -> SqliteClient {
        SqliteClient {
            scorelog_db_url,
            score_db_url,
            song_db_url,
        }
    }

    pub fn by_config() -> SqliteClient {
        SqliteClient::new(
            config::config().scorelog_db_url(),
            config::config().song_db_url(),
            config::config().score_db_url(),
        )
    }

    fn establish_connection(url: &str) -> Result<SqliteConnection, diesel::ConnectionError> {
        SqliteConnection::establish(&url)
    }

    fn score_log(&self) -> HashMap<SongId, SnapShots> {
        use schema::score_log::scorelog::dsl::*;
        let connection = Self::establish_connection(&self.scorelog_db_url).unwrap();
        let record: Vec<schema::score_log::ScoreLog> =
            scorelog.load(&connection).expect("Error loading schema");

        let mut map = HashMap::new();
        for row in record {
            let song_id = SongId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode));
            let snap =
                SnapShot::from_data(row.clear, row.score, row.combo, row.minbp, row.date as i64);
            map.entry(song_id).or_insert(SnapShots::default()).add(snap);
        }
        map
    }

    pub fn player(&self) -> PlayerStates {
        use schema::player::player::dsl::*;
        let connection = Self::establish_connection(&self.score_db_url).unwrap();
        let records: Vec<schema::player::Player> = player
            .load::<schema::player::Player>(&connection)
            .expect("Error loading schema");

        let mut log = Vec::new();
        for row in records {
            let pl = PlayerState::new(
                PlayCount::new(row.playcount),
                PlayCount::new(row.clear),
                PlayTime::new(row.playtime),
                UpdatedAt::from_timestamp(row.date as i64),
                TotalJudge::new(Judge::new(
                    row.epg, row.lpg, row.egr, row.lgr, row.egd, row.lgd, row.ebd, row.lbd,
                    row.epr, row.lpr, row.ems, row.lms,
                )),
            );
            log.push(pl);
        }
        PlayerStates::new(log)
    }

    pub fn song_data(&self) -> Result<Songs> {
        let connection = Self::establish_connection(&self.song_db_url)?;
        let record: Vec<schema::song::Song> = schema::song::song::table.load(&connection)?;

        Ok(record
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
            .build())
    }
}

impl ScoreRepository for SqliteClient {
    fn score(&self) -> Scores {
        let connection = Self::establish_connection(&self.score_db_url).unwrap();
        let record = schema::score::score::table
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
                            UpdatedAt::from_timestamp(row.date as i64),
                            Judge::new(
                                row.epg, row.lpg, row.egr, row.lgr, row.egd, row.lgd, row.ebd,
                                row.lbd, row.epr, row.lpr, row.ems, row.lms,
                            ),
                            MaxCombo::from_combo(row.combo),
                            PlayCount::new(row.playcount),
                            ClearCount::new(row.clearcount),
                            MinBP::from_bp(row.minbp),
                            score_log
                                .get(&song_id)
                                .unwrap_or(&SnapShots::default())
                                .clone(),
                        ),
                    )
                })
                .collect::<HashMap<SongId, Score>>(),
        )
    }

    fn save_score(&self, _account: Account, _score: Scores) -> Result<()> {
        unimplemented!()
    }
}
