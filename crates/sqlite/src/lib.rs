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

    fn establish_connection(url: &str) -> Result<SqliteConnection, diesel::ConnectionError> {
        SqliteConnection::establish(&url)
    }

    fn score_log(&self) -> HashMap<ScoreId, SnapShots> {
        use schema::score_log::scorelog::dsl::*;
        let connection = Self::establish_connection(&self.scorelog_db_url).unwrap();
        let record: Vec<schema::score_log::ScoreLog> =
            scorelog.load(&connection).expect("Error loading schema");

        record.iter().fold(HashMap::new(), |mut map, row| {
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
        })
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
                    IncludeFeatures::from(row.feature),
                );
                builder
            })
            .build())
    }
    pub fn score(&self) -> Scores {
        let connection = Self::establish_connection(&self.score_db_url).unwrap();
        let record = schema::score::score::table
            .load::<schema::score::Score>(&connection)
            .expect("Error loading schema");
        let score_log = self.score_log();
        Scores::create_by_map(
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
        )
    }
}
