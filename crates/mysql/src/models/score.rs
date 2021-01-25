use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "scores"]
pub struct Score {
    pub id: i32,
    pub user_id: i32,
    pub sha256: String,
    pub mode: i32,
    pub clear: i32,
    pub epg: i32,
    pub lpg: i32,
    pub egr: i32,
    pub lgr: i32,
    pub egd: i32,
    pub lgd: i32,
    pub ebd: i32,
    pub lbd: i32,
    pub epr: i32,
    pub lpr: i32,
    pub ems: i32,
    pub lms: i32,
    pub combo: i32,
    pub min_bp: i32,
    pub play_count: i32,
    pub clear_count: i32,
    pub date: NaiveDateTime,
}

impl CanGetHash for Score {
    fn hash_sha256(&self) -> String {
        self.sha256.clone()
    }
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "scores"]
pub struct RegisteredScore {
    pub user_id: i32,
    pub sha256: String,
    pub mode: i32,
    pub clear: i32,
    pub epg: i32,
    pub lpg: i32,
    pub egr: i32,
    pub lgr: i32,
    pub egd: i32,
    pub lgd: i32,
    pub ebd: i32,
    pub lbd: i32,
    pub epr: i32,
    pub lpr: i32,
    pub ems: i32,
    pub lms: i32,
    pub combo: i32,
    pub min_bp: i32,
    pub play_count: i32,
    pub clear_count: i32,
    pub date: NaiveDateTime,
}

impl CanGetHash for RegisteredScore {
    fn hash_sha256(&self) -> String {
        self.sha256.clone()
    }
}

pub trait CanGetHash {
    fn hash_sha256(&self) -> String;
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "score_snaps"]
pub struct ScoreSnap {
    pub id: i32,
    pub user_id: i32,
    pub sha256: String,
    pub mode: i32,
    pub date: NaiveDateTime,
    pub clear: i32,
    pub score: i32,
    pub combo: i32,
    pub min_bp: i32,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "score_snaps"]
pub struct ScoreSnapForUpdate {
    pub user_id: i32,
    pub sha256: String,
    pub mode: i32,
    pub date: NaiveDateTime,
    pub clear: i32,
    pub score: i32,
    pub combo: i32,
    pub min_bp: i32,
}

impl CanGetHash for ScoreSnapForUpdate {
    fn hash_sha256(&self) -> String {
        self.sha256.clone()
    }
}
