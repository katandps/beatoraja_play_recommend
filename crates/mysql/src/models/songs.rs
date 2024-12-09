use crate::models::DieselResult;
use crate::schema::*;
use crate::MySqlPooledConnection;
use diesel::prelude::*;
use itertools::Itertools;

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = songs)]
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

    pub fn all(connection: &mut MySqlPooledConnection) -> DieselResult<Vec<Self>> {
        use crate::schema::songs::dsl::*;
        songs.load(connection)
    }

    pub fn by_hashes(
        connection: &mut MySqlPooledConnection,
        sha256list: &[&str],
    ) -> DieselResult<Vec<Self>> {
        use crate::schema::songs::dsl::*;
        let result = sha256list
            .into_iter()
            .chunks(1000)
            .into_iter()
            .map(|chunk| {
                let hash_list = chunk.cloned().collect::<Vec<_>>();
                songs.filter(sha256.eq_any(hash_list)).load(connection)
            })
            .collect();
        Self::unroll(result)
    }
    fn unroll(items: Vec<DieselResult<Vec<Self>>>) -> DieselResult<Vec<Self>> {
        items.into_iter().process_results(|i| i.flatten().collect())
    }
}
