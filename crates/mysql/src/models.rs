use crate::schema::hashes;
use crate::schema::songs;
use crate::schema::users;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "songs"]
pub struct Song {
    pub sha256: String,
    pub title: String,
    pub subtitle: String,
    pub artist: String,
    pub sub_artist: String,
    pub notes: i32,
    pub length: i32,
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "hashes"]
pub struct Hash {
    pub sha256: String,
    pub md5: String,
}

#[derive(Debug, Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub gmail_address: String,
    pub name: String,
    pub registered_date: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "users"]
pub struct RegisteringUser {
    pub gmail_address: String,
    pub name: String,
    pub registered_date: NaiveDateTime,
}
