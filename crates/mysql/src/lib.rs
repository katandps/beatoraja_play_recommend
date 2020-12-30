mod models;
mod schema;

use crate::models::{CanGetHash, RegisteredScore, ScoreSnapForUpdate};
use anyhow::Result;
use chrono::Utc;
use diesel::prelude::*;
use model::gmail_address::GmailAddress;
use model::google_id::GoogleId;
use model::registered_date::RegisteredDate;
use model::user_name::UserName;
use model::*;
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate diesel;
extern crate anyhow;

pub struct MySQLClient {
    connection: MysqlConnection,
}

impl MySQLClient {
    pub fn new() -> Self {
        Self {
            connection: Self::establish_connection(config().mysql_url()),
        }
    }

    fn establish_connection(url: String) -> MysqlConnection {
        MysqlConnection::establish(&url).expect(&format!("Error connecting to {}", url))
    }

    fn songs(&self) -> Vec<models::Song> {
        schema::songs::table
            .load(&self.connection)
            .expect("Error loading schema")
    }

    fn hash(&self) -> Vec<models::Hash> {
        schema::hashes::table
            .load(&self.connection)
            .expect("Error loading schema")
    }

    pub fn register(&self, profile: &GoogleProfile) -> Result<Account> {
        let user: Vec<models::User> = schema::users::table
            .filter(schema::users::gmail_address.eq(&profile.email))
            .load(&self.connection)
            .expect("Error loading schema");

        if user.is_empty() {
            let user = models::RegisteringUser {
                google_id: profile.user_id.clone(),
                gmail_address: profile.email.clone(),
                name: profile.name.to_string(),
                registered_date: Utc::now().naive_utc(),
            };
            println!("Insert new user");
            diesel::insert_into(schema::users::table)
                .values(user.clone())
                .execute(&self.connection)?;
            self.get_account(profile)
        } else {
            let user = user[0].clone();
            Ok(Account::new(
                GoogleId::new(user.google_id),
                GmailAddress::new(user.gmail_address),
                UserName::new(user.name),
                RegisteredDate::new(user.registered_date),
            ))
        }
    }

    pub fn account_by_increments(&self, id: i32) -> Result<Account> {
        let user: models::User = schema::users::table
            .filter(schema::users::id.eq(id))
            .first(&self.connection)?;

        Ok(Account::new(
            GoogleId::new(user.google_id),
            GmailAddress::new(user.gmail_address),
            UserName::new(user.name),
            RegisteredDate::new(user.registered_date),
        ))
    }

    pub fn account_by_id(&self, google_id: GoogleId) -> Result<Account> {
        let user: models::User = schema::users::table
            .filter(schema::users::google_id.eq(google_id.to_string()))
            .first(&self.connection)?;

        Ok(Account::new(
            GoogleId::new(user.google_id),
            GmailAddress::new(user.gmail_address),
            UserName::new(user.name),
            RegisteredDate::new(user.registered_date),
        ))
    }

    pub fn save_account(&self, account: Account) -> Result<()> {
        let mut user: models::User = schema::users::table
            .filter(schema::users::gmail_address.eq(account.email()))
            .first(&self.connection)?;

        user.name = account.name.to_string();
        println!("Update user name.");
        diesel::replace_into(schema::users::table)
            .values(user.clone())
            .execute(&self.connection)?;
        Ok(())
    }

    fn get_account(&self, profile: &GoogleProfile) -> Result<Account> {
        let user: models::User = schema::users::table
            .filter(schema::users::gmail_address.eq(&profile.email))
            .first(&self.connection)?;

        Ok(Account::new(
            GoogleId::new(user.google_id),
            GmailAddress::new(user.gmail_address),
            UserName::new(user.name),
            RegisteredDate::new(user.registered_date),
        ))
    }

    fn score_log(&self) -> HashMap<SongId, SnapShots> {
        let records: Vec<models::ScoreSnap> = schema::score_snaps::table
            .load(&self.connection)
            .expect("Error loading schema");
        let mut map = HashMap::new();
        for row in records {
            let song_id = SongId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode));
            let snap = SnapShot::from_data(
                row.clear,
                row.score,
                row.combo,
                row.min_bp,
                row.date.timestamp() as i32,
            );
            map.entry(song_id).or_insert(SnapShots::default()).add(snap);
        }
        map
    }

    pub fn score(&self, account: Account) -> Result<Scores> {
        let user: models::User = schema::users::table
            .filter(schema::users::gmail_address.eq(account.email()))
            .first(&self.connection)?;
        let record: Vec<models::Score> = schema::scores::table
            .filter(schema::scores::user_id.eq(user.id))
            .load(&self.connection)?;
        let score_log = self.score_log();
        Ok(Scores::new(
            record
                .iter()
                .map(|row| {
                    let song_id = SongId::new(row.sha256.parse().unwrap(), PlayMode::new(row.mode));
                    (
                        song_id.clone(),
                        Score::new(
                            ClearType::from_integer(row.clear),
                            UpdatedAt::from_timestamp(row.date.timestamp() as i32),
                            Judge::new(
                                row.epg, row.lpg, row.egr, row.lgr, row.egd, row.lgd, row.ebd,
                                row.lbd, row.epr, row.lpr, row.ems, row.lms,
                            ),
                            MaxCombo::from_combo(row.combo),
                            PlayCount::new(row.play_count),
                            ClearCount::new(row.clear_count),
                            MinBP::from_bp(row.min_bp),
                            score_log
                                .get(&song_id)
                                .unwrap_or(&SnapShots::default())
                                .clone(),
                        ),
                    )
                })
                .collect::<HashMap<SongId, Score>>(),
        ))
    }

    pub fn save_score(&self, account: Account, score: Scores) -> Result<()> {
        let user = schema::users::table
            .filter(schema::users::gmail_address.eq(account.email()))
            .first::<models::User>(&self.connection)?;
        let user_id = user.id;
        let saved: Vec<models::Score> = schema::scores::table
            .filter(schema::scores::user_id.eq(user_id))
            .load(&self.connection)?;
        let saved_song = saved
            .iter()
            .map(|record| {
                (
                    SongId::new(
                        HashSha256::new(record.sha256.clone()),
                        PlayMode::new(record.mode),
                    ),
                    record,
                )
            })
            .collect::<HashMap<_, _>>();
        let saved: Vec<models::ScoreSnap> = schema::score_snaps::table
            .filter(schema::score_snaps::user_id.eq(user_id))
            .load(&self.connection)?;

        let saved_snap = saved
            .iter()
            .map(|record| {
                (
                    (
                        SongId::new(
                            HashSha256::new(record.sha256.clone()),
                            PlayMode::new(record.mode),
                        ),
                        record.date.clone(),
                    ),
                    record,
                )
            })
            .collect::<HashMap<_, _>>();

        let hashes = self
            .hash()
            .iter()
            .map(|h| h.sha256.clone())
            .collect::<HashSet<_>>();

        let mut songs_for_insert = Vec::new();
        let mut songs_for_update = Vec::new();
        let mut snaps_for_insert = Vec::new();

        for (song_id, score) in score.0 {
            match saved_song.get(&song_id) {
                Some(saved) => {
                    if UpdatedAt::from_str(&saved.date.to_string()) < score.updated_at {
                        songs_for_update.push(models::Score {
                            id: saved.id,
                            user_id,
                            sha256: song_id.sha256().to_string(),
                            mode: song_id.mode().0,
                            clear: score.clear.to_integer(),
                            epg: score.judge.early_pgreat,
                            lpg: score.judge.late_pgreat,
                            egr: score.judge.early_great,
                            lgr: score.judge.late_great,
                            egd: score.judge.early_good,
                            lgd: score.judge.late_good,
                            ebd: score.judge.early_bad,
                            lbd: score.judge.late_bad,
                            epr: score.judge.early_poor,
                            lpr: score.judge.late_poor,
                            ems: score.judge.early_miss,
                            lms: score.judge.late_miss,
                            combo: score.max_combo.0,
                            min_bp: score.min_bp.0,
                            play_count: score.play_count.0,
                            clear_count: 0,
                            date: score.updated_at.naive_datetime(),
                        })
                    }
                }
                None => songs_for_insert.push(RegisteredScore {
                    user_id,
                    sha256: song_id.sha256().to_string(),
                    mode: song_id.mode().0,
                    clear: score.clear.to_integer(),
                    epg: score.judge.early_pgreat,
                    lpg: score.judge.late_pgreat,
                    egr: score.judge.early_great,
                    lgr: score.judge.late_great,
                    egd: score.judge.early_good,
                    lgd: score.judge.late_good,
                    ebd: score.judge.early_bad,
                    lbd: score.judge.late_bad,
                    epr: score.judge.early_poor,
                    lpr: score.judge.late_poor,
                    ems: score.judge.early_miss,
                    lms: score.judge.late_miss,
                    combo: score.max_combo.0,
                    min_bp: score.min_bp.0,
                    play_count: score.play_count.0,
                    clear_count: 0,
                    date: score.updated_at.naive_datetime(),
                }),
            };
            for snapshot in &score.log.0 {
                match saved_snap.get(&(song_id.clone(), snapshot.updated_at.naive_datetime())) {
                    Some(_) => (),
                    None => snaps_for_insert.push(ScoreSnapForUpdate {
                        user_id,
                        sha256: song_id.sha256().to_string(),
                        mode: song_id.mode().0,
                        date: snapshot.updated_at.naive_datetime(),
                        clear: snapshot.clear_type.to_integer(),
                        score: snapshot.score.ex_score(),
                        combo: snapshot.max_combo.0,
                        min_bp: snapshot.min_bp.0,
                    }),
                }
            }
        }
        dbg!(&songs_for_insert.len());
        dbg!(&songs_for_update.len());
        dbg!(&snaps_for_insert.len());
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

    pub fn save_song(&self, songs: &Songs) -> Result<()> {
        let exist_hashes = self.hash();
        let mut hashmap = songs.converter.sha256_to_md5.clone();
        for row in exist_hashes {
            hashmap.remove(&HashSha256::new(row.sha256));
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
            while index < new_hashes.len() && records.len() < 100 {
                records.push(new_hashes[index].clone());
                index += 1;
            }
            if records.is_empty() {
                break;
            }
            println!("Insert {} hashes.", records.len());
            diesel::insert_into(schema::hashes::table)
                .values(records)
                .execute(&self.connection)?;
        }

        let exist_songs = self.songs();
        let mut songs = songs.songs.clone();
        for row in exist_songs {
            songs.remove(&HashSha256::new(row.sha256));
        }
        let new_songs = songs
            .iter()
            .map(|(_, song)| models::Song {
                sha256: song.hash.to_string(),
                title: song.title.to_string(),
                subtitle: "".into(),
                artist: song.artist.to_string(),
                sub_artist: "".into(),
                notes: song.notes,
                length: 0,
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
            println!("Insert {} songs.", records.len());
            diesel::insert_into(schema::songs::table)
                .values(records)
                .execute(&self.connection)?;
        }
        Ok(())
    }

    pub fn song_data(&self) -> Songs {
        let record = self.songs();
        let hash: HashMap<String, String> = self
            .hash()
            .iter()
            .map(|hash| (hash.sha256.clone(), hash.md5.clone()))
            .collect();

        record
            .iter()
            .fold(SongsBuilder::new(), |mut builder, row| {
                let md5 = HashMd5::new(hash.get(&row.sha256).unwrap().clone());
                builder.push(
                    md5,
                    HashSha256::new(row.sha256.clone()),
                    Title::new(format!("{}{}", row.title, row.subtitle)),
                    Artist::new(row.artist.clone()),
                    row.notes,
                );
                builder
            })
            .build()
    }
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
