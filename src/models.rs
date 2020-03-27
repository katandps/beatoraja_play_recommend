use super::schema::player::player;
use diesel::Queryable;

type DB = diesel::sqlite::Sqlite;

#[derive(Queryable)]
pub struct Player {
    pub date: i32,
    pub playcount: i32,
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
    pub playtime: i32,
}

pub struct Score {
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
    pub notes: i32,
    pub combo: i32,
    pub minbp: i32,
    pub playcount: i32,
    pub clearcount: i32,
    pub history: i32,
    pub scorehash: String,
    pub option: i32,
    pub random: i32,
    pub date: i32,
    pub state: i32,
    pub trophy: String,
    pub ghost: String,
}