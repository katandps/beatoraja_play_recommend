use crate::models::{DieselResult, User};
use crate::schema::*;
use crate::MySqlPooledConnection;
use chrono::NaiveDateTime;
use model::{Judge, PlayCount, PlayTime, TotalJudge, UpdatedAt};

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = player_stats)]
pub struct PlayerStat {
    pub id: i32,
    pub user_id: i32,
    pub date: NaiveDateTime,
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

impl PlayerStat {
    pub fn by_user_id(
        connection: &mut MySqlPooledConnection,
        query_id: i32,
    ) -> DieselResult<Vec<PlayerStat>> {
        use crate::schema::player_stats::dsl::*;
        player_stats.filter(user_id.eq(query_id)).load(connection)
    }

    pub fn to_stat(&self) -> model::PlayerStat {
        model::PlayerStat {
            play_count: PlayCount::new(self.playcount),
            clear_count: PlayCount::new(self.clear),
            play_time: PlayTime::new(self.playtime),
            date: UpdatedAt::from_timestamp(self.date.timestamp()),
            total_judge: TotalJudge::new(Judge {
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
            }),
        }
    }

    pub fn delete_by_user(
        connection: &mut MySqlPooledConnection,
        user: &User,
    ) -> DieselResult<usize> {
        use crate::schema::player_stats::dsl::*;
        diesel::delete(player_stats.filter(user_id.eq(user.id))).execute(connection)
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = player_stats)]
pub struct PlayerStatForInsert {
    pub user_id: i32,
    pub date: NaiveDateTime,
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
#[diesel(table_name = player_stats)]
pub struct PlayerStatForUpdate {
    pub id: i32,
    pub user_id: i32,
    pub date: NaiveDateTime,
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
