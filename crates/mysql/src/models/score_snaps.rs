use crate::models::{CanGetHash, DieselResult};
use crate::schema::*;
use crate::MySqlPooledConnection;
use chrono::NaiveDateTime;

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

impl ScoreSnap {
    pub fn by_user_id(
        connection: &MySqlPooledConnection,
        query_id: i32,
    ) -> DieselResult<Vec<ScoreSnap>> {
        use crate::schema::score_snaps::dsl::*;
        score_snaps.filter(user_id.eq(query_id)).load(connection)
    }
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
