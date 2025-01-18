use crate::models::DieselResult;
use crate::{schema::*, MySqlPooledConnection};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use model::{UploadAt, UserId};

#[derive(Debug, Clone, Queryable)]
#[diesel(table_name = score_upload_logs)]
#[allow(unused)]
pub struct ScoreUpload {
    pub id: i32,
    pub user_id: i32,
    pub date: NaiveDateTime,
}

impl ScoreUpload {
    pub fn last_by_user_id(
        connection: &mut MySqlPooledConnection,
        query_id: i32,
    ) -> DieselResult<Self> {
        use crate::schema::score_upload_logs::dsl::*;
        score_upload_logs
            .filter(user_id.eq(query_id))
            .order_by(date)
            .first(connection)
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = score_upload_logs)]
pub struct RegisteringScoreLog {
    pub user_id: i32,
    pub date: NaiveDateTime,
}

impl RegisteringScoreLog {
    pub fn new(user_id: UserId, upload_at: UploadAt) -> RegisteringScoreLog {
        RegisteringScoreLog {
            user_id: user_id.get(),
            date: upload_at.0.naive_utc(),
        }
    }
}
