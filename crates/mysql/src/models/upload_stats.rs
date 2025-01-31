use crate::schema::*;
use diesel::prelude::*;

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
