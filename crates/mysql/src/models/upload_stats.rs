use crate::models::DieselResult;
use crate::schema::*;
use crate::MySqlPooledConnection;
use diesel::prelude::*;
use model::Judge;

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = upload_log_stats)]
pub struct UploadStats {
    pub id: i32,
    pub upload_log_id: i32,
    pub user_id: i32,
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

impl UploadStats {
    pub fn list_by_user_id(
        connection: &mut MySqlPooledConnection,
        query_user_id: i32,
    ) -> DieselResult<Vec<Self>> {
        use crate::schema::upload_log_stats::dsl::*;
        upload_log_stats
            .filter(user_id.eq(query_user_id))
            .load(connection)
    }

    pub fn judge(&self) -> Judge {
        Judge {
            early_pgreat: self.epg,
            late_pgreat: self.lpg,
            early_great: self.egr,
            late_great: self.lgr,
            early_good: self.egd,
            late_good: self.lgd,
            early_bad: self.ebd,
            late_bad: self.lbd,
            early_poor: self.epr,
            late_poor: self.lpr,
            early_miss: self.ems,
            late_miss: self.lms,
        }
    }

    pub fn by_upload_id_and_user_id(
        connection: &mut MySqlPooledConnection,
        query_upload_id: i32,
        query_user_id: i32,
    ) -> DieselResult<Option<Self>> {
        use crate::schema::upload_log_stats::dsl::*;
        upload_log_stats
            .filter(upload_log_id.eq(query_upload_id))
            .filter(user_id.eq(query_user_id))
            .first(connection)
            .optional()
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = upload_log_stats)]
pub struct UploadStatsForInsert {
    pub upload_log_id: i32,
    pub user_id: i32,
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
