mod config;
mod error;
mod models;
mod schema;

pub use crate::error::Error;
use crate::models::PlayerStatForUpdate;
use crate::models::{
    CanGetHash, Hash, PlayerStatForInsert, ScoreSnapForUpdate, User, UserStatus,
    UserStatusForInsert,
};
use anyhow::Result;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error::NotFound;
use diesel::MysqlConnection;
use model::*;
use models::UploadStatsForInsert;
use oauth_google::{GoogleProfile, RegisterUser};
use r2d2::Pool;
use repository::*;
use schema::revoked_sessions;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub type MySqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn get_db_pool() -> MySqlPool {
    Pool::builder().build_unchecked(ConnectionManager::new(&config::config().mysql_url))
}

pub struct MySQLClient {
    connection: MySqlPooledConnection,
}

impl MySQLClient {
    pub fn new(connection: MySqlPooledConnection) -> Self {
        Self { connection }
    }

    fn score_log(&mut self, account: &Account) -> Result<HashMap<ScoreId, SnapShots>, Error> {
        let records = models::ScoreSnap::by_user_id(&mut self.connection, account.user_id().get())?;
        let mut map: HashMap<ScoreId, SnapShots> = HashMap::new();
        for row in records {
            let song_id = ScoreId::new(row.sha256.parse().unwrap(), PlayMode::from(row.mode));
            let snap = SnapShot::from_data(
                row.clear,
                row.score,
                row.combo,
                row.min_bp,
                row.date.and_utc().timestamp(),
            );
            map.entry(song_id).or_default().add(snap);
        }
        Ok(map)
    }

    fn score_log_by_sha256(
        &mut self,
        sha256: &HashSha256,
    ) -> Result<HashMap<UserId, SnapShots>, Error> {
        let records = models::ScoreSnap::by_sha256(&mut self.connection, &sha256.to_string())?;
        let mut map: HashMap<UserId, SnapShots> = HashMap::new();
        for row in records {
            let user_id = UserId::new(row.user_id);
            let snap = SnapShot::from_data(
                row.clear,
                row.score,
                row.combo,
                row.min_bp,
                row.date.and_utc().timestamp(),
            );
            map.entry(user_id).or_default().add(snap);
        }
        Ok(map)
    }

    fn saved_song(&mut self, user_id: i32) -> Result<HashMap<ScoreId, models::Score>> {
        let saved = models::Score::by_user_id(&mut self.connection, user_id)?;
        let map = saved
            .into_iter()
            .map(|record| (record.get_score_id(), record))
            .collect::<HashMap<_, _>>();
        Ok(map)
    }

    fn saved_snap(
        &mut self,
        user_id: i32,
    ) -> Result<HashMap<(ScoreId, NaiveDateTime), models::ScoreSnap>> {
        let saved = models::ScoreSnap::by_user_id(&mut self.connection, user_id)?;
        Ok(saved
            .into_iter()
            .map(|record| ((record.get_score_id(), record.date), record))
            .collect::<HashMap<_, _>>())
    }
}

impl HealthCheck for MySQLClient {
    async fn health(&mut self) -> Result<()> {
        Ok(User::count_users(&mut self.connection).map(|_| ())?)
    }
}

impl RegisterUser for MySQLClient {
    async fn register(&mut self, profile: &GoogleProfile) -> Result<()> {
        let user = User::by_google_profile(&mut self.connection, profile);
        match user {
            Ok(_) => Ok(()),
            Err(_) => {
                use crate::schema::users::dsl::*;
                log::info!("Insert new user: {}", profile.email);
                diesel::insert_into(users)
                    .values(models::RegisteringUser::from_profile(profile))
                    .execute(&mut self.connection)?;
                Ok(())
            }
        }
    }
}

impl AccountByUserId for MySQLClient {
    async fn user(&mut self, id: i32) -> Result<Account> {
        Ok(User::by_user_id(&mut self.connection, id)?.into())
    }
}

impl AccountByGoogleId for MySQLClient {
    async fn user(&mut self, google_id: &GoogleId) -> Result<Account> {
        Ok(User::by_google_id(&mut self.connection, google_id.to_string())?.into())
    }
}

impl RenameAccount for MySQLClient {
    async fn rename(&mut self, account: &Account) -> Result<()> {
        log::info!("Update user name to {}.", account.user_name());
        let user = User::by_account(&mut self.connection, account)?;
        diesel::insert_into(schema::rename_logs::table)
            .values(models::RenameUser {
                user_id: user.id,
                old_name: user.name,
                new_name: account.user_name(),
                date: Utc::now().naive_utc(),
            })
            .execute(&mut self.connection)?;

        diesel::update(
            schema::users::table.filter(schema::users::gmail_address.eq(account.email())),
        )
        .set(schema::users::name.eq(account.user_name()))
        .execute(&mut self.connection)?;

        Ok(())
    }
}

impl ChangeAccountVisibility for MySQLClient {
    async fn change_visibility(&mut self, account: &Account) -> Result<()> {
        log::info!(
            "Update visibility to {}. : {}",
            account.visibility,
            account.user_id().get()
        );
        let user = User::by_account(&mut self.connection, account)?;
        let user_status = UserStatus::by_user(&mut self.connection, &user);
        match user_status {
            Ok(status) => {
                use crate::schema::user_statuses::dsl::*;
                diesel::update(user_statuses.filter(id.eq(status.id)))
                    .set(visible.eq(account.visibility()))
                    .execute(&mut self.connection)?;
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
                    .execute(&mut self.connection)?;
            }
        }
        Ok(())
    }
}

impl AllSongData for MySQLClient {
    async fn song_data(&mut self) -> Result<Songs> {
        let record = models::Song::all(&mut self.connection)?;
        let hash = Hash::all(&mut self.connection)?;
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

impl SongDataForTables for MySQLClient {
    async fn song_data(&mut self, tables: &Tables) -> Result<Songs> {
        let hash = Hash::for_tables(
            &tables
                .get_charts()
                .map(|c| c.md5().as_str())
                .collect::<Vec<_>>(),
            &mut self.connection,
        )?;
        let record = models::Song::by_hashes(
            &mut self.connection,
            &hash
                .iter()
                .map(|hash| hash.sha256.as_str())
                .collect::<Vec<_>>(),
        )?;
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
    async fn save_score(
        &mut self,
        account: &Account,
        score: &Scores,
        upload: &ScoreUpload,
    ) -> Result<()> {
        let user = User::by_account(&mut self.connection, account).unwrap();
        let user_id = user.id;
        let saved_song = self.saved_song(user_id).unwrap();
        let saved_snap = self.saved_snap(user_id).unwrap();

        let hashes = Hash::all(&mut self.connection)
            .unwrap()
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
                        songs_for_update.push(models::Score::from_score(
                            saved,
                            score,
                            user_id,
                            song_id,
                            &upload.upload_id,
                        ))
                    }
                }
                None => songs_for_insert.push(models::RegisteredScore::from_score(
                    user_id,
                    score,
                    song_id,
                    &upload.upload_id,
                )),
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
                        score_upload_log_id: Some(upload.upload_id.0),
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
            diesel::replace_into(schema::scores::table)
                .values(v)
                .execute(&mut self.connection)
                .unwrap();
        }

        for v in div(songs_for_insert, &hashes) {
            log::info!("Insert {} scores.", v.len());
            diesel::insert_into(schema::scores::table)
                .values(v)
                .execute(&mut self.connection)
                .unwrap();
        }

        for v in div(snaps_for_insert, &hashes) {
            log::info!("Insert {} score_snaps", v.len());
            diesel::insert_into(schema::score_snaps::table)
                .values(v)
                .execute(&mut self.connection)
                .unwrap();
        }

        Ok(())
    }
}

impl SaveSongData for MySQLClient {
    async fn save_song(&mut self, songs: &Songs) -> Result<()> {
        let exist_hashes = Hash::all(&mut self.connection)?;
        let mut hashmap = songs.converter.sha256_to_md5.clone();
        let mut songs = songs.songs.clone();

        for hash in &exist_hashes {
            let _ = HashSha256::from_str(&hash.sha256).map(|hash| {
                hashmap.remove(&hash);
                songs.remove(&hash);
            });
        }
        let new_hashes = hashmap
            .iter()
            .map(|(sha256, md5)| Hash {
                sha256: sha256.to_string(),
                md5: md5.to_string(),
            })
            .collect::<Vec<_>>();
        Hash::insert_new_hashes(new_hashes, &mut self.connection)?;
        let new_songs = songs
            .values()
            .map(models::Song::from_song)
            .collect::<Vec<_>>();
        for records in new_songs.chunks(100) {
            log::info!("Insert {} songs.", records.len());
            diesel::replace_into(schema::songs::table)
                .values(records)
                .execute(&mut self.connection)?;
        }
        Ok(())
    }
}

impl SavePlayerStateData for MySQLClient {
    async fn save_player_states(
        &mut self,
        account: &Account,
        stats: &PlayerStats,
        upload: &ScoreUpload,
    ) -> Result<()> {
        let user = User::by_account(&mut self.connection, account)?;

        let saved = models::PlayerStat::by_user_id(&mut self.connection, user.id)?
            .into_iter()
            .map(|s| (s.date, s))
            .collect::<HashMap<_, _>>();

        let mut inserts = Vec::new();
        let mut updates = Vec::new();
        for stat in stats.log.iter() {
            if let Some(saved) = saved.get(&stat.date.naive_datetime()) {
                if saved.playcount < stat.play_count.0 {
                    updates.push(PlayerStatForUpdate {
                        id: saved.id,
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
                    });
                }
            } else {
                inserts.push(PlayerStatForInsert {
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
        if let Some(last) = stats.last() {
            let stat = UploadStatsForInsert {
                upload_log_id: upload.upload_id.0,
                user_id: user.id,
                playcount: last.play_count.0,
                clear: last.clear_count.0,
                epg: last.total_judge.judge().early_pgreat,
                lpg: last.total_judge.judge().late_pgreat,
                egr: last.total_judge.judge().early_great,
                lgr: last.total_judge.judge().late_great,
                egd: last.total_judge.judge().early_good,
                lgd: last.total_judge.judge().late_good,
                ebd: last.total_judge.judge().early_bad,
                lbd: last.total_judge.judge().late_bad,
                epr: last.total_judge.judge().early_poor,
                lpr: last.total_judge.judge().late_poor,
                ems: last.total_judge.judge().early_miss,
                lms: last.total_judge.judge().late_miss,
                playtime: last.play_time.0,
            };
            log::info!("Insert stat this time");
            diesel::replace_into(schema::upload_log_stats::table)
                .values(stat)
                .execute(&mut self.connection)?;
        }
        log::info!("Save stat for {} days", inserts.len());
        diesel::insert_into(schema::player_stats::table)
            .values(inserts)
            .execute(&mut self.connection)?;
        log::info!("Update stat on {} days", updates.len());
        diesel::replace_into(schema::player_stats::table)
            .values(updates)
            .execute(&mut self.connection)?;
        Ok(())
    }
}

impl StatsByAccount for MySQLClient {
    async fn stats(&mut self, account: &Account) -> Result<PlayerStats> {
        let user = User::by_account(&mut self.connection, account)?;
        let record = models::PlayerStat::by_user_id(&mut self.connection, user.id)?;
        Ok(PlayerStats::new(
            record.into_iter().map(|row| row.to_stat()).collect(),
        ))
    }
}

impl ScoresByAccount for MySQLClient {
    async fn score(&mut self, account: &Account) -> Result<Scores> {
        let user = User::by_account(&mut self.connection, account)?;
        let record = models::Score::by_user_id(&mut self.connection, user.id)?;
        let mut score_log = self.score_log(account)?;
        Ok(Scores::create_by_map(
            record
                .into_iter()
                .filter_map(|row| {
                    row.sha256
                        .parse()
                        .map(|sha256| {
                            let score_id = ScoreId::new(sha256, PlayMode::from(row.mode));
                            let log = score_log.remove(&score_id).unwrap_or_default();
                            (score_id, row.to_score().with_log(log))
                        })
                        .ok()
                })
                .collect::<HashMap<ScoreId, Score>>(),
        ))
    }
}

impl ScoreByAccountAndSha256 for MySQLClient {
    async fn score_with_log(&mut self, account: &Account, score_id: &ScoreId) -> Result<Score> {
        let user = User::by_account(&mut self.connection, account)?;
        let record = models::Score::by_user_id_and_score_id(
            &mut self.connection,
            user.id,
            &score_id.sha256().to_string(),
            score_id.mode().to_int(),
        )?;
        let score = record.first().ok_or(NotFound)?.to_score();
        let snaps = {
            let records = models::ScoreSnap::by_user_id_and_score_id(
                &mut self.connection,
                user.id,
                &score_id.sha256().to_string(),
                score_id.mode().to_int(),
            )?;
            let mut snapshots = SnapShots::default();
            for row in records {
                let snap = SnapShot::from_data(
                    row.clear,
                    row.score,
                    row.combo,
                    row.min_bp,
                    row.date.and_utc().timestamp(),
                );
                snapshots.add(snap);
            }
            snapshots
        };
        Ok(score.with_log(snaps))
    }
}

impl ScoresBySha256 for MySQLClient {
    async fn score(&mut self, hash: &HashSha256) -> Result<RankedScore> {
        let record = models::Score::by_sha256(&mut self.connection, &hash.to_string())?;
        let mut score_log = self.score_log_by_sha256(hash)?;
        Ok(RankedScore::create_by_map(
            record
                .into_iter()
                .map(|row| {
                    let user_id = UserId::new(row.user_id);
                    let log = score_log.remove(&user_id).unwrap_or_default();
                    (user_id, row.to_score().with_log(log))
                })
                .collect::<HashMap<UserId, Score>>(),
        ))
    }
}

impl PublishedUsers for MySQLClient {
    async fn fetch_users(&mut self) -> Result<Vec<VisibleAccount>> {
        let list = UserStatus::visible_with_account(&mut self.connection)?;
        let mut res = Vec::new();
        for (_status, user) in list {
            res.push(VisibleAccount {
                id: UserId::new(user.id),
                name: user.name,
            })
        }
        Ok(res)
    }
}

impl ResetScore for MySQLClient {
    async fn reset_score(&mut self, account: &Account) -> Result<()> {
        let user = User::by_account(&mut self.connection, account)?;
        models::Score::delete_by_user(&mut self.connection, &user)?;
        models::ScoreSnap::delete_by_user(&mut self.connection, &user)?;
        UserStatus::delete_by_user(&mut self.connection, &user)?;
        models::PlayerStat::delete_by_user(&mut self.connection, &user)?;
        log::info!(
            "Score data is removed: account id = {}",
            account.user_id.get()
        );
        Ok(())
    }
}

impl RegisterUpload for MySQLClient {
    async fn register_upload(
        &mut self,
        user_id: UserId,
        upload_at: UploadAt,
    ) -> Result<ScoreUpload> {
        let record = models::ScoreUpload::last_by_user_id(&mut self.connection, user_id.get());
        match record {
            Ok(record) if upload_at.0.naive_utc() - record.date < Duration::minutes(10) => {
                log::info!(
                    "already registered score: {}: {}: {}",
                    user_id.get(),
                    upload_at.0,
                    upload_at.0.naive_utc() - record.date
                );
                Ok(ScoreUpload::new(
                    UploadId(record.id),
                    UploadAt(record.date.and_utc()),
                ))
            }
            _ => {
                use crate::schema::score_upload_logs;
                log::info!("register new scores: {}: {}", user_id.get(), upload_at.0);
                let record = self
                    .connection
                    .transaction(|connection| {
                        diesel::insert_into(score_upload_logs::table)
                            .values(models::RegisteringScoreLog::new(user_id, upload_at.clone()))
                            .execute(connection)
                            .unwrap();
                        models::ScoreUpload::last_by_user_id(connection, user_id.get())
                    })
                    .unwrap();

                Ok(ScoreUpload::new(
                    UploadId(record.id),
                    UploadAt(record.date.and_utc()),
                ))
            }
        }
    }
}

impl RevokeSession for MySQLClient {
    async fn is_revoked(&mut self, session_key: &SessionKey, user_id: UserId) -> Result<bool> {
        let revokes = models::RevokedSession::revoked(
            &mut self.connection,
            session_key.as_str(),
            user_id.get(),
        )?;
        Ok(!revokes.is_empty())
    }
    async fn revoke(&mut self, session_key: &SessionKey, user_id: UserId) -> Result<()> {
        let record = models::SessionRevoke::new(session_key, user_id);
        diesel::insert_into(revoked_sessions::table)
            .values(record)
            .execute(&mut self.connection)?;
        Ok(())
    }
}
