mod schema;

use diesel::prelude::*;
use model::*;
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

#[macro_use]
extern crate diesel;

pub struct SqliteClient {
    scorelog_db_url: String,
    song_db_url: String,
    score_db_url: String,
}

impl SqliteClient {
    fn new(scorelog_db_url: String, song_db_url: String, score_db_url: String) -> SqliteClient {
        SqliteClient {
            scorelog_db_url,
            score_db_url,
            song_db_url,
        }
    }

    pub fn for_score(score_db_url: &str, scorelog_db_url: &str) -> SqliteClient {
        SqliteClient::new(scorelog_db_url.into(), "".into(), score_db_url.into())
    }

    pub fn for_song(song_db_url: &str) -> SqliteClient {
        SqliteClient::new("".into(), song_db_url.into(), "".into())
    }

    fn establish_connection(url: &str) -> SqliteResult<SqliteConnection> {
        let connection = SqliteConnection::establish(url)?;
        Ok(connection)
    }

    fn score_log(&self) -> Result<HashMap<ScoreId, SnapShots>, SqliteError> {
        use schema::score_log::scorelog::dsl::*;
        let connection = Self::establish_connection(&self.scorelog_db_url)?;
        let record: Vec<schema::score_log::ScoreLog> = scorelog.load(&connection)?;

        Ok(record.into_iter().fold(HashMap::new(), |mut map, row| {
            map.entry(ScoreId::new(
                row.sha256.parse().unwrap(),
                PlayMode::from(row.mode),
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

    pub fn player(&self) -> SqliteResult<PlayerStats> {
        use schema::player::player::dsl::*;
        let connection = Self::establish_connection(&self.score_db_url)?;
        let records: Vec<schema::player::Player> = player.load(&connection)?;

        let mut log = Vec::new();
        for row in records {
            let pl = PlayerStat::new(
                PlayCount::new(row.playcount),
                PlayCount::new(row.clear),
                PlayTime::new(row.playtime),
                UpdatedAt::from_timestamp(row.date as i64),
                TotalJudge::new(Judge {
                    early_pgreat: row.epg,
                    late_pgreat: row.lpg,
                    early_great: row.egr,
                    late_great: row.lgr,
                    early_good: row.egd,
                    late_good: row.lgd,
                    early_bad: row.ebd,
                    late_bad: row.lbd,
                    early_poor: row.epr,
                    late_poor: row.lpr,
                    early_miss: row.ems,
                    late_miss: row.lms,
                }),
            );
            log.push(pl);
        }
        Ok(PlayerStats::new(log))
    }

    pub fn song_data(&self) -> Result<Songs, SqliteError> {
        let connection = Self::establish_connection(&self.song_db_url)?;
        let record: Vec<schema::song::Song> = schema::song::song::table.load(&connection)?;

        Ok(record
            .iter()
            .fold(SongsBuilder::default(), |mut builder, row| {
                builder.push(
                    HashMd5::from_str(&row.md5).unwrap(),
                    HashSha256::from_str(&row.sha256).unwrap(),
                    Title::new(format!("{}{}", row.title, row.subtitle)),
                    Artist::new(row.artist.clone()),
                    row.notes,
                    IncludeFeatures::from(row.feature),
                );
                builder
            })
            .build())
    }

    pub fn score(&self) -> SqliteResult<Scores> {
        let connection = Self::establish_connection(&self.score_db_url)?;
        let record: Vec<schema::score::Score> = schema::score::score::table.load(&connection)?;
        let score_log = self.score_log()?;
        Ok(Scores::create_by_map(
            record
                .iter()
                .map(|row| {
                    let song_id =
                        ScoreId::new(row.sha256.parse().unwrap(), PlayMode::from(row.mode));
                    (
                        song_id.clone(),
                        Score::new(
                            ClearType::from_integer(row.clear),
                            UpdatedAt::from_timestamp(row.date as i64),
                            Judge {
                                early_pgreat: row.epg,
                                late_pgreat: row.lpg,
                                early_great: row.egr,
                                late_great: row.lgr,
                                early_good: row.egd,
                                late_good: row.lgd,
                                early_bad: row.ebd,
                                late_bad: row.lbd,
                                early_poor: row.epr,
                                late_poor: row.lpr,
                                early_miss: row.ems,
                                late_miss: row.lms,
                            },
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

pub type SqliteResult<T> = Result<T, SqliteError>;

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
