mod score;
mod user;

use crate::schema::*;
use crate::MySqlPooledConnection;
pub use score::*;
pub use user::*;

pub(crate) type DieselResult<T> = Result<T, diesel::result::Error>;

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

impl Song {
    pub fn from_song(song: &model::Song) -> Self {
        Self {
            sha256: song.get_sha256().to_string(),
            title: song.title(),
            subtitle: "".into(),
            artist: song.artist(),
            sub_artist: "".into(),
            notes: song.notes(),
            length: 0,
            features: song.features().clone().into(),
        }
    }

    pub fn all(connection: &MySqlPooledConnection) -> DieselResult<Vec<Self>> {
        use crate::schema::songs::dsl::*;
        songs.load(connection)
    }
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "hashes"]
pub struct Hash {
    pub sha256: String,
    pub md5: String,
}

impl Hash {
    pub fn all(connection: &MySqlPooledConnection) -> DieselResult<Vec<Self>> {
        use crate::schema::hashes::dsl::*;
        hashes.load(connection)
    }
}
