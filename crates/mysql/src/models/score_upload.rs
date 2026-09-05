use crate::models::DieselResult;
use crate::{schema::*, MySqlPooledConnection};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use model::{PlayerStat, PlayerStatDiff, UploadAt, UserId};

#[derive(Debug, Clone, Queryable)]
#[diesel(table_name = score_upload_logs)]
#[allow(unused)]
pub struct ScoreUpload {
    pub id: i32,
    pub user_id: i32,
    pub date: NaiveDateTime,
}

impl ScoreUpload {
    fn latest_by_user_id_query(
        query_id: i32,
    ) -> crate::schema::score_upload_logs::BoxedQuery<'static, diesel::mysql::Mysql> {
        use crate::schema::score_upload_logs::dsl::*;
        score_upload_logs
            .filter(user_id.eq(query_id))
            .order_by(id.desc())
            .into_boxed()
    }

    pub fn last_by_user_id(
        connection: &mut MySqlPooledConnection,
        query_id: i32,
    ) -> DieselResult<Self> {
        Self::latest_by_user_id_query(query_id).first(connection)
    }

    pub fn list_by_user_id(
        connection: &mut MySqlPooledConnection,
        query_id: i32,
    ) -> DieselResult<Vec<Self>> {
        use crate::schema::score_upload_logs::dsl::*;
        score_upload_logs
            .filter(user_id.eq(query_id))
            .order_by(id.desc())
            .load(connection)
    }

    pub fn by_user_id_and_upload_id(
        connection: &mut MySqlPooledConnection,
        query_user_id: i32,
        query_upload_id: i32,
    ) -> DieselResult<Option<Self>> {
        use crate::schema::score_upload_logs::dsl::*;
        score_upload_logs
            .filter(user_id.eq(query_user_id))
            .filter(id.eq(query_upload_id))
            .first(connection)
            .optional()
    }

    pub fn prev_by_user_id_before_upload_id(
        connection: &mut MySqlPooledConnection,
        query_user_id: i32,
        query_upload_id: i32,
    ) -> DieselResult<Option<Self>> {
        use crate::schema::score_upload_logs::dsl::*;
        score_upload_logs
            .filter(user_id.eq(query_user_id))
            .filter(id.lt(query_upload_id))
            .order_by(id.desc())
            .first(connection)
            .optional()
    }

    pub fn to_score_upload(
        self,
        song_count: i64,
        stats: PlayerStatDiff,
        total_stats: PlayerStat,
    ) -> model::ScoreUpload {
        model::ScoreUpload::with_stats(
            model::UploadId(self.id),
            UploadAt(self.date.and_utc()),
            song_count,
            stats,
            total_stats,
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_by_user_id_orders_upload_logs_descending() {
        let query = ScoreUpload::latest_by_user_id_query(1);
        let sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();

        assert!(sql.contains("ORDER BY `score_upload_logs`.`id` DESC"));
    }
}
