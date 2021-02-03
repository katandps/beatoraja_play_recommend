mod config;
mod error;
mod models;
mod query;
mod schema;

pub use crate::error::Error;
use crate::models::{CanGetHash, ScoreSnapForUpdate};
use anyhow::anyhow;
use anyhow::Result;
use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::MysqlConnection;
use model::*;
use oauth_google::GoogleProfile;
use r2d2::Pool;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

pub type MySqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub struct MySQLClient {
    connection: MySqlPooledConnection,
}

pub fn get_db_pool() -> MySqlPool {
    Pool::builder().build_unchecked(ConnectionManager::new(config::config().mysql_url))
}

impl MySQLClient {
    pub fn new(connection: MySqlPooledConnection) -> Self {
        Self { connection }
    }

    pub fn health(&self) -> Result<()> {
        match &self.connection.execute("SELECT 1") {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow!("HealthCheckError")),
        }
    }

    pub fn register(&self, profile: &GoogleProfile) -> Result<Account> {
        let user = query::account_by_email(&self.connection, &profile.email);
        match user {
            Ok(user) => Ok(Self::create_account(user)),
            Err(_) => {
                let user = models::RegisteringUser {
                    google_id: profile.user_id.clone(),
                    gmail_address: profile.email.clone(),
                    name: profile.name.to_string(),
                    registered_date: Utc::now().naive_utc(),
                };
                log::info!("Insert new user: {}", profile.email);
                diesel::insert_into(schema::users::table)
                    .values(user.clone())
                    .execute(&self.connection)?;
                Ok(Self::create_account(query::account_by_email(
                    &self.connection,
                    &profile.email,
                )?))
            }
        }
    }

    pub fn account_by_increments(&self, id: i32) -> Result<Account> {
        Ok(Self::create_account(query::account_by_id(
            &self.connection,
            id,
        )?))
    }

    fn create_account(model: models::User) -> Account {
        Account::new(
            UserId::new(model.id),
            GoogleId::new(model.google_id),
            GmailAddress::new(model.gmail_address),
            UserName::new(model.name),
            RegisteredDate::new(model.registered_date),
            Visibility::new(true),
        )
    }

    pub fn account_by_id(&self, google_id: GoogleId) -> Result<Account, Error> {
        Ok(Self::create_account(query::account_by_google_id(
            &self.connection,
            &google_id.to_string(),
        )?))
    }

    pub fn rename_account(&self, account: &Account) -> Result<(), Error> {
        log::info!("Update user name to {}.", account.user_name());
        let user = query::account_by_email(&self.connection, &account.email())?;
        diesel::insert_into(schema::rename_logs::table)
            .values(models::RenameUser {
                user_id: user.id.clone(),
                old_name: user.name.clone(),
                new_name: account.user_name(),
                date: Utc::now().naive_utc(),
            })
            .execute(&self.connection)?;

        diesel::update(
            schema::users::table.filter(schema::users::gmail_address.eq(account.email())),
        )
        .set(schema::users::name.eq(account.user_name()))
        .execute(&self.connection)?;

        Ok(())
    }

    pub fn change_account_visibility(&self, _account: &Account) -> Result<(), Error> {
        Ok(())
    }

    fn score_log(&self, account: &Account) -> Result<HashMap<ScoreId, SnapShots>, Error> {
        let records = query::score_snaps_by_user_id(&self.connection, account.user_id())?;
        let mut map = HashMap::new();
        for row in records {
            let song_id = ScoreId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode));
            let snap = SnapShot::from_data(
                row.clear,
                row.score,
                row.combo,
                row.min_bp,
                row.date.timestamp(),
            );
            map.entry(song_id).or_insert(SnapShots::default()).add(snap);
        }
        Ok(map)
    }

    pub fn score(&self, account: &Account) -> Result<Scores, Error> {
        let user = query::account_by_email(&self.connection, &account.email())?;
        let record = query::scores_by_user_id(&self.connection, user.id)?;
        let score_log = self.score_log(account)?;
        Ok(Scores::create_by_map(
            record
                .iter()
                .filter_map(|row| Self::make_score(row, &score_log).ok())
                .collect::<HashMap<ScoreId, Score>>(),
        ))
    }

    fn make_score(
        score: &models::Score,
        log: &HashMap<ScoreId, SnapShots>,
    ) -> Result<(ScoreId, Score)> {
        let song_id = ScoreId::new(score.sha256.parse()?, PlayMode::new(score.mode));
        Ok((
            song_id.clone(),
            Score::new(
                ClearType::from_integer(score.clear),
                UpdatedAt::from_timestamp(score.date.timestamp()),
                Judge::new(
                    score.epg, score.lpg, score.egr, score.lgr, score.egd, score.lgd, score.ebd,
                    score.lbd, score.epr, score.lpr, score.ems, score.lms,
                ),
                MaxCombo::from_combo(score.combo),
                PlayCount::new(score.play_count),
                ClearCount::new(score.clear_count),
                MinBP::from_bp(score.min_bp),
                log.get(&song_id).cloned().unwrap_or_default(),
            ),
        ))
    }

    pub fn save_score(&self, account: Account, score: Scores) -> Result<()> {
        let user = query::account_by_email(&self.connection, &account.email())?;
        let user_id = user.id;
        let saved = query::scores_by_user_id(&self.connection, user.id)?;
        let saved_song = saved
            .iter()
            .map(|record| {
                (
                    ScoreId::new(
                        HashSha256::from_str(&record.sha256).unwrap(),
                        PlayMode::new(record.mode),
                    ),
                    record,
                )
            })
            .collect::<HashMap<_, _>>();
        let saved = query::score_snaps_by_user_id(&self.connection, user.id)?;
        let saved_snap = saved
            .iter()
            .map(|record| {
                (
                    (
                        ScoreId::new(
                            HashSha256::from_str(&record.sha256).unwrap(),
                            PlayMode::new(record.mode),
                        ),
                        record.date.clone(),
                    ),
                    record,
                )
            })
            .collect::<HashMap<_, _>>();

        let hashes = query::hashes(&self.connection)?
            .iter()
            .map(|h| h.sha256.clone())
            .collect::<HashSet<_>>();

        let mut songs_for_insert = Vec::new();
        let mut songs_for_update = Vec::new();
        let mut snaps_for_insert = Vec::new();

        for (song_id, score) in score.get_map() {
            match saved_song.get(&song_id) {
                Some(saved) => {
                    if UpdatedAt::from_naive_datetime(saved.date) < score.updated_at {
                        songs_for_update
                            .push(models::Score::from_score(saved, score, user_id, song_id))
                    }
                }
                None => songs_for_insert
                    .push(models::RegisteredScore::from_score(user_id, score, song_id)),
            };
            for snapshot in &score.log.0 {
                match saved_snap.get(&(song_id.clone(), snapshot.updated_at.naive_datetime())) {
                    Some(_) => (),
                    None => snaps_for_insert.push(ScoreSnapForUpdate {
                        user_id,
                        sha256: song_id.sha256().to_string(),
                        mode: song_id.mode().to_int(),
                        date: snapshot.updated_at.naive_datetime(),
                        clear: snapshot.clear_type.to_integer(),
                        score: snapshot.score.ex_score(),
                        combo: snapshot.max_combo.0,
                        min_bp: snapshot.min_bp.0,
                    }),
                }
            }
        }
        log::info!("Songs for Insert {} records.", songs_for_insert.len());
        log::info!("Songs for Update {} records.", songs_for_update.len());
        log::info!("Snaps for Insert {} records.", snaps_for_insert.len());
        fn div<T: Clone + CanGetHash>(v: Vec<T>, hashes: &HashSet<String>) -> Vec<Vec<T>> {
            let mut index = 0;
            let mut ret = Vec::new();
            loop {
                let mut records = Vec::new();
                while index < v.len() && records.len() < 1000 {
                    if hashes.contains(&v[index].hash_sha256()) {
                        records.push(v[index].clone());
                    }
                    index += 1;
                }
                if records.is_empty() {
                    break;
                }
                ret.push(records);
            }
            ret
        }

        for v in div(songs_for_update, &hashes) {
            println!("Update {} scores.", v.len());
            let _result = diesel::replace_into(schema::scores::table)
                .values(v)
                .execute(&self.connection);
        }

        for v in div(songs_for_insert, &hashes) {
            println!("Insert {} scores.", v.len());
            diesel::insert_into(schema::scores::table)
                .values(v)
                .execute(&self.connection)?;
        }

        for v in div(snaps_for_insert, &hashes) {
            println!("Insert {} score_snaps", v.len());
            diesel::insert_into(schema::score_snaps::table)
                .values(v)
                .execute(&self.connection)?;
        }

        Ok(())
    }

    pub fn save_song(&self, songs: &Songs) -> Result<(), Error> {
        let exist_hashes = query::hashes(&self.connection)?;
        let mut hashmap = songs.converter.sha256_to_md5.clone();
        for row in exist_hashes {
            let _ = HashSha256::from_str(&row.sha256).map(|hash| hashmap.remove(&hash));
        }
        let new_hashes = hashmap
            .iter()
            .map(|(sha256, md5)| models::Hash {
                sha256: sha256.to_string(),
                md5: md5.to_string(),
            })
            .collect::<Vec<_>>();

        let mut index = 0;
        loop {
            let mut records = Vec::new();
            while index < new_hashes.len() && records.len() < 1000 {
                records.push(new_hashes[index].clone());
                index += 1;
            }
            if records.is_empty() {
                break;
            }
            log::info!("Insert {} hashes.", records.len());
            diesel::insert_into(schema::hashes::table)
                .values(records)
                .execute(&self.connection)?;
        }

        let exist_songs = query::songs(&self.connection)?;
        let mut songs = songs.songs.clone();
        for row in exist_songs {
            let _ = HashSha256::from_str(&row.sha256).map(|hash| songs.remove(&hash));
        }
        let new_songs = songs
            .iter()
            .map(|(_, song)| models::Song {
                sha256: song.get_sha256().to_string(),
                title: song.title(),
                subtitle: "".into(),
                artist: song.artist(),
                sub_artist: "".into(),
                notes: song.notes(),
                length: 0,
                features: song.features().clone().into(),
            })
            .collect::<Vec<_>>();
        let mut index = 0;
        loop {
            let mut records = Vec::new();
            while index < new_songs.len() && records.len() < 100 {
                records.push(new_songs[index].clone());
                index += 1;
            }
            if records.is_empty() {
                break;
            }
            log::info!("Insert {} songs.", records.len());
            diesel::insert_into(schema::songs::table)
                .values(records)
                .execute(&self.connection)?;
        }
        Ok(())
    }

    pub fn song_data(&self) -> Result<Songs> {
        let record = query::songs(&self.connection)?;
        let hash = query::hashes(&self.connection)?;
        let hash = hash
            .iter()
            .map(|hash| (&hash.sha256, &hash.md5))
            .collect::<HashMap<&String, &String>>();

        Ok(record
            .iter()
            .fold(SongsBuilder::new(), |mut builder, row| {
                builder.push(
                    HashMd5::from_str(hash.get(&row.sha256).unwrap()).unwrap(),
                    HashSha256::from_str(&row.sha256).unwrap(),
                    Title::new(format!("{}{}", row.title, row.subtitle)),
                    Artist::new(row.artist.clone()),
                    row.notes,
                    IncludeFeatures::from(row.features),
                );
                builder
            })
            .build())
    }
}
