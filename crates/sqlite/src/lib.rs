mod schema;

use diesel::prelude::*;
use model::*;
use std::collections::HashMap;
use thiserror::Error;

#[macro_use]
extern crate diesel;

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

    fn establish_connection(url: &str) -> Result<SqliteConnection, diesel::ConnectionError> {
        SqliteConnection::establish(&url)
    }

    fn score_log(&self) -> Result<HashMap<ScoreId, SnapShots>, SqliteError> {
        use schema::score_log::scorelog::dsl::*;
        let connection = Self::establish_connection(&self.scorelog_db_url)?;
        let record: Vec<schema::score_log::ScoreLog> = scorelog.load(&connection)?;

        Ok(record.iter().fold(HashMap::new(), |mut map, row| {
            map.entry(ScoreId::new(
                row.sha256.parse().unwrap(),
                PlayMode::new(row.mode),
            ))
            .or_default()
            .add(SnapShot::from_data(
                row.clear,
                row.score,
                row.combo,
                row.minbp,
                row.date as i64,
            ));
            map
        }))
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

    pub fn song_data(&self) -> Result<Songs, SqliteError> {
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
                    IncludeFeatures::from(row.feature),
                );
                builder
            })
            .build())
    }
    pub fn score(&self) -> Result<Scores, SqliteError> {
        let connection = Self::establish_connection(&self.score_db_url)?;
        let record = schema::score::score::table.load::<schema::score::Score>(&connection)?;
        let score_log = self.score_log()?;
        Ok(Scores::create_by_map(
            record
                .iter()
                .map(|row| {
                    let song_id =
                        ScoreId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode));
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
                            score_log.get(&song_id).cloned().unwrap_or_default(),
                        ),
                    )
                })
                .collect::<HashMap<ScoreId, Score>>(),
        ))
    }
}

#[derive(Debug, Error)]
pub enum SqliteError {
    #[error("ConnectionError {0:?}")]
    ConnectionError(diesel::ConnectionError),
    #[error("DieselResultError {0:?}")]
    DieselResultError(diesel::result::Error),
}

impl From<diesel::ConnectionError> for SqliteError {
    fn from(e: ConnectionError) -> Self {
        SqliteError::ConnectionError(e)
    }
}
impl From<diesel::result::Error> for SqliteError {
    fn from(e: diesel::result::Error) -> Self {
        SqliteError::DieselResultError(e)
    }
}
