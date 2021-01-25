mod score;
mod user;

use crate::schema::*;
pub use score::*;
pub use user::*;

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
    pub features: i32,
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "hashes"]
pub struct Hash {
    pub sha256: String,
    pub md5: String,
}
