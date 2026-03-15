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

    pub fn by_user_id_and_upload_id(
        connection: &mut MySqlPooledConnection,
        _query_user_id: i32,
        query_upload_id: i32,
    ) -> DieselResult<Self> {
        use crate::schema::score_upload_logs::dsl::*;
        score_upload_logs
            .filter(id.eq(query_upload_id))
            .first(connection)
    }

    pub fn prev_by_user_id_before_upload_id(
        connection: &mut MySqlPooledConnection,
        query_user_id: i32,
        query_upload_id: i32,
    ) -> DieselResult<Self> {
        use crate::schema::score_upload_logs::dsl::*;
        score_upload_logs
            .filter(user_id.eq(query_user_id))
            .filter(id.lt(query_upload_id))
            .order_by(id.desc())
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
