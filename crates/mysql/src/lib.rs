mod config;
mod error;
mod models;
mod schema;

pub use crate::error::Error;
use crate::models::{
    CanGetHash, Hash, PlayerStatForUpdate, ScoreSnapForUpdate, User, UserStatus,
    UserStatusForInsert,
};
use anyhow::anyhow;
use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::MysqlConnection;
use model::*;
use oauth_google::{GoogleProfile, RegisterUser};
use r2d2::Pool;
use repository::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

pub type MySqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn get_db_pool() -> MySqlPool {
    Pool::builder().build_unchecked(ConnectionManager::new(config::config().mysql_url))
}

pub struct MySQLClient {
    connection: MySqlPooledConnection,
}

impl MySQLClient {
    pub fn new(connection: MySqlPooledConnection) -> Self {
        Self { connection }
    }

    fn score_log(&self, account: &Account) -> Result<HashMap<ScoreId, SnapShots>, Error> {
        let records = models::ScoreSnap::by_user_id(&self.connection, account.user_id().get())?;
        let mut map: HashMap<ScoreId, SnapShots> = HashMap::new();
        for row in records {
            let song_id = ScoreId::new(row.sha256.parse().unwrap(), PlayMode::from(row.mode));
            let snap = SnapShot::from_data(
                row.clear,
                row.score,
                row.combo,
                row.min_bp,
                row.date.timestamp(),
            );
            map.entry(song_id).or_default().add(snap);
        }
        Ok(map)
    }

    fn score_log_by_sha256(
        &self,
        sha256: &HashSha256,
    ) -> Result<HashMap<UserId, SnapShots>, Error> {
        let records = models::ScoreSnap::by_sha256(&self.connection, &sha256.to_string())?;
        let mut map: HashMap<UserId, SnapShots> = HashMap::new();
        for row in records {
            let user_id = UserId::new(row.user_id);
            let snap = SnapShot::from_data(
                row.clear,
                row.score,
                row.combo,
                row.min_bp,
                row.date.timestamp(),
            );
            map.entry(user_id).or_default().add(snap);
        }
        Ok(map)
    }

    fn saved_song(&self, user_id: i32) -> Result<HashMap<ScoreId, models::Score>> {
        let saved = models::Score::by_user_id(&self.connection, user_id)?;
        let map = saved
            .into_iter()
            .map(|record| (record.get_score_id(), record))
            .collect::<HashMap<_, _>>();
        Ok(map)
    }

    fn saved_snap(
        &self,
        user_id: i32,
    ) -> Result<HashMap<(ScoreId, NaiveDateTime), models::ScoreSnap>> {
        let saved = models::ScoreSnap::by_user_id(&self.connection, user_id)?;
        Ok(saved
            .into_iter()
            .map(|record| ((record.get_score_id(), record.date), record))
            .collect::<HashMap<_, _>>())
    }
}

impl HealthCheck for MySQLClient {
    fn health(&self) -> Result<()> {
        match &self.connection.execute("SELECT 1") {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow!("HealthCheckError")),
        }
    }
}

impl RegisterUser for MySQLClient {
    fn register(&self, profile: &GoogleProfile) -> Result<()> {
        let user = User::by_google_profile(&self.connection, profile);
        match user {
            Ok(_) => Ok(()),
            Err(_) => {
                use crate::schema::users::dsl::*;
                log::info!("Insert new user: {}", profile.email);
                diesel::insert_into(users)
                    .values(models::RegisteringUser::from_profile(profile))
                    .execute(&self.connection)?;
                Ok(())
            }
        }
    }
}

impl AccountByUserId for MySQLClient {
    fn user(&self, id: i32) -> Result<Account> {
        Ok(User::by_user_id(&self.connection, id)?.into())
    }
}

impl AccountByGoogleId for MySQLClient {
    fn user(&self, google_id: &GoogleId) -> Result<Account> {
        Ok(User::by_google_id(&self.connection, google_id.to_string())?.into())
    }
}

impl RenameAccount for MySQLClient {
    fn rename(&self, account: &Account) -> Result<()> {
        log::info!("Update user name to {}.", account.user_name());
        let user = User::by_account(&self.connection, account)?;
        diesel::insert_into(schema::rename_logs::table)
            .values(models::RenameUser {
                user_id: user.id,
                old_name: user.name,
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
}

impl ChangeAccountVisibility for MySQLClient {
    fn change_visibility(&self, account: &Account) -> Result<()> {
        log::info!(
            "Update visibility to {}. : {}",
            account.visibility,
            account.user_id().get()
        );
        let user = User::by_account(&self.connection, account)?;
        let user_status = UserStatus::by_user(&self.connection, &user);
        match user_status {
            Ok(status) => {
                use crate::schema::user_statuses::dsl::*;
                diesel::update(user_statuses.filter(id.eq(status.id)))
                    .set(visible.eq(account.visibility()))
                    .execute(&self.connection)?;
            }
            Err(_) => {
                use crate::schema::user_statuses::dsl::*;
                let new = UserStatusForInsert {
                    user_id: user.id,
                    visible: account.visibility(),
                    score_updated_at: Utc::now().naive_utc(),
                };
                diesel::insert_into(user_statuses)
                    .values(new)
                    .execute(&self.connection)?;
            }
        }
        Ok(())
    }
}

impl AllSongData for MySQLClient {
    fn song_data(&self) -> Result<Songs> {
        let record = models::Song::all(&self.connection)?;
        let hash = Hash::all(&self.connection)?;
        let hash = hash
            .iter()
            .map(|hash| (&hash.sha256, &hash.md5))
            .collect::<HashMap<&String, &String>>();

        Ok(record
            .iter()
            .fold(SongsBuilder::default(), |mut builder, row| {
                builder.push(
                    HashMd5::from_str(hash.get(&row.sha256).unwrap()).unwrap(),
                    HashSha256::from_str(&row.sha256).unwrap(),
                    Title::from_title_and_subtitle(&row.title, &row.subtitle),
                    Artist::new(row.artist.clone()),
                    row.notes,
                    IncludeFeatures::from(row.features),
                );
                builder
            })
            .build())
    }
}

impl SaveScoreData for MySQLClient {
    fn save_score(&self, account: &Account, score: &Scores) -> Result<()> {
        let user = User::by_account(&self.connection, account)?;
        let user_id = user.id;
        let saved_song = self.saved_song(user_id)?;
        let saved_snap = self.saved_snap(user_id)?;

        let hashes = Hash::all(&self.connection)?
            .iter()
            .map(|h| h.sha256.clone())
            .collect::<HashSet<_>>();

        let mut songs_for_insert = Vec::new();
        let mut songs_for_update = Vec::new();
        let mut snaps_for_insert = Vec::new();

        for (song_id, score) in score.get_map() {
            match saved_song.get(song_id) {
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
            log::info!("Update {} scores.", v.len());
            let _result = diesel::replace_into(schema::scores::table)
                .values(v)
                .execute(&self.connection);
        }

        for v in div(songs_for_insert, &hashes) {
            log::info!("Insert {} scores.", v.len());
            diesel::insert_into(schema::scores::table)
                .values(v)
                .execute(&self.connection)?;
        }

        for v in div(snaps_for_insert, &hashes) {
            log::info!("Insert {} score_snaps", v.len());
            diesel::insert_into(schema::score_snaps::table)
                .values(v)
                .execute(&self.connection)?;
        }

        Ok(())
    }
}

impl SaveSongData for MySQLClient {
    fn save_song(&self, songs: &Songs) -> Result<()> {
        let exist_hashes = Hash::all(&self.connection)?;
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

        let exist_songs = models::Song::all(&self.connection)?;
        let mut songs = songs.songs.clone();
        for row in exist_songs {
            let _ = HashSha256::from_str(&row.sha256).map(|hash| songs.remove(&hash));
        }
        let new_songs = songs
            .iter()
            .map(|(_, song)| models::Song::from_song(song))
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
}

impl SavePlayerStateData for MySQLClient {
    fn save_player_states(&self, account: &Account, stats: &PlayerStats) -> Result<()> {
        let user = User::by_account(&self.connection, account)?;

        let saved = models::PlayerStat::by_user_id(&self.connection, user.id)?
            .into_iter()
            .map(|s| (s.date.clone(), s))
            .collect::<HashMap<_, _>>();

        let mut inserts = Vec::new();
        let mut updates = Vec::new();
        for stat in stats.log.iter() {
            if let Some(saved) = saved.get(&stat.date.naive_datetime()) {
                if saved.playcount < stat.play_count.0 {
                    updates.push(saved.clone());
                }
            } else {
                inserts.push(PlayerStatForUpdate {
                    user_id: user.id,
                    date: stat.date.naive_datetime(),
                    playcount: stat.play_count.0,
                    clear: stat.clear_count.0,
                    epg: stat.total_judge.judge().early_pgreat,
                    lpg: stat.total_judge.judge().late_pgreat,
                    egr: stat.total_judge.judge().early_great,
                    lgr: stat.total_judge.judge().late_great,
                    egd: stat.total_judge.judge().early_good,
                    lgd: stat.total_judge.judge().late_good,
                    ebd: stat.total_judge.judge().early_bad,
                    lbd: stat.total_judge.judge().late_bad,
                    epr: stat.total_judge.judge().early_poor,
                    lpr: stat.total_judge.judge().late_poor,
                    ems: stat.total_judge.judge().early_miss,
                    lms: stat.total_judge.judge().late_miss,
                    playtime: stat.play_time.0,
                })
            }
        }
        log::info!("Save stat for {} days", inserts.len());
        diesel::insert_into(schema::player_stats::table)
            .values(inserts)
            .execute(&self.connection)?;
        log::info!("Update stat on {} days", updates.len());
        diesel::replace_into(schema::player_stats::table)
            .values(updates)
            .execute(&self.connection)?;
        Ok(())
    }
}

impl StatsByAccount for MySQLClient {
    fn stats(&self, account: &Account) -> Result<PlayerStats> {
        let user = User::by_account(&self.connection, account)?;
        let record = models::PlayerStat::by_user_id(&self.connection, user.id)?;
        Ok(PlayerStats::new(
            record.into_iter().map(|row| row.to_stat()).collect(),
        ))
    }
}

impl ScoresByAccount for MySQLClient {
    fn score(&self, account: &Account) -> Result<Scores> {
        let user = User::by_account(&self.connection, account)?;
        let record = models::Score::by_user_id(&self.connection, user.id)?;
        let score_log = self.score_log(account)?;
        Ok(Scores::create_by_map(
            record
                .into_iter()
                .filter_map(|row| {
                    let sha256 = row.sha256.parse();
                    if let Ok(sha256) = sha256 {
                        let score_id = ScoreId::new(sha256, PlayMode::from(row.mode));
                        let log = score_log.get(&score_id).cloned().unwrap_or_default();
                        Some((score_id, row.to_score().with_log(log)))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<ScoreId, Score>>(),
        ))
    }
}

impl ScoresBySha256 for MySQLClient {
    fn score(&self, hash: &HashSha256) -> Result<RankedScore> {
        let record = models::Score::by_sha256(&self.connection, &hash.to_string())?;
        let score_log = self.score_log_by_sha256(hash)?;
        Ok(RankedScore::create_by_map(
            record
                .into_iter()
                .map(|row| {
                    let user_id = UserId::new(row.user_id);
                    let log = score_log.get(&user_id).cloned().unwrap_or_default();
                    (user_id, row.to_score().with_log(log))
                })
                .collect::<HashMap<UserId, Score>>(),
        ))
    }
}

impl PublishedUsers for MySQLClient {
    fn fetch_users(&self) -> Result<Vec<VisibleAccount>> {
        let list = UserStatus::visible_with_account(&self.connection)?;
        let mut res = Vec::new();
        for (_status, user) in list {
            res.push(VisibleAccount {
                id: user.id,
                name: user.name,
            })
        }
        Ok(res)
    }
}
