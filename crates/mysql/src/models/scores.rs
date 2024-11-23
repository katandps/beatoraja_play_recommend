use crate::models::{DieselResult, User};
use crate::schema::*;
use crate::MySqlPooledConnection;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use model::{
    ClearCount, ClearType, HashSha256, Judge, MaxCombo, MinBP, PlayCount, PlayMode, ScoreId,
    SnapShots, UpdatedAt, UploadId,
};
use std::str::FromStr;

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = scores)]
pub struct Score {
    pub id: i32,
    pub user_id: i32,
    pub sha256: String,
    pub mode: i32,
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
    pub combo: i32,
    pub min_bp: i32,
    pub play_count: i32,
    pub clear_count: i32,
    pub date: NaiveDateTime,
    pub score_upload_log_id: Option<i32>,
}

impl Score {
    pub fn by_user_id(
        connection: &mut MySqlPooledConnection,
        query_id: i32,
    ) -> DieselResult<Vec<Self>> {
        use crate::schema::scores::dsl::*;
        scores.filter(user_id.eq(query_id)).load(connection)
    }

    pub fn delete_by_user(
        connection: &mut MySqlPooledConnection,
        user: &User,
    ) -> DieselResult<usize> {
        use crate::schema::scores::dsl::*;
        diesel::delete(scores.filter(user_id.eq(user.id))).execute(connection)
    }

    pub fn by_sha256(
        connection: &mut MySqlPooledConnection,
        query_hash: &str,
    ) -> DieselResult<Vec<Self>> {
        use crate::schema::scores::dsl::*;
        scores.filter(sha256.eq(query_hash)).load(connection)
    }

    pub fn by_user_id_and_score_id(
        connection: &mut MySqlPooledConnection,
        query_id: i32,
        query_hash: &str,
        query_mode: i32,
    ) -> DieselResult<Vec<Self>> {
        use crate::schema::scores::dsl::*;
        scores
            .filter(user_id.eq(query_id))
            .filter(sha256.eq(query_hash))
            .filter(mode.eq(query_mode))
            .load(connection)
    }

    pub fn from_score(
        saved: &Self,
        score: &model::Score,
        user_id: i32,
        song_id: &ScoreId,
        score_upload_log_id: &UploadId,
    ) -> Self {
        Self {
            id: saved.id,
            user_id,
            sha256: song_id.sha256().to_string(),
            mode: song_id.mode().to_int(),
            clear: score.clear.to_integer(),
            epg: score.judge.early_pgreat,
            lpg: score.judge.late_pgreat,
            egr: score.judge.early_great,
            lgr: score.judge.late_great,
            egd: score.judge.early_good,
            lgd: score.judge.late_good,
            ebd: score.judge.early_bad,
            lbd: score.judge.late_bad,
            epr: score.judge.early_poor,
            lpr: score.judge.late_poor,
            ems: score.judge.early_miss,
            lms: score.judge.late_miss,
            combo: score.max_combo.0,
            min_bp: score.min_bp.0,
            play_count: score.play_count.0,
            clear_count: 0,
            score_upload_log_id: Some(score_upload_log_id.0),
            date: score.updated_at.naive_datetime(),
        }
    }

    pub fn to_score(&self) -> model::Score {
        let judge = Judge {
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
        };
        model::Score {
            clear: ClearType::from_integer(self.clear),
            updated_at: UpdatedAt::from_timestamp(self.date.and_utc().timestamp()),
            score: judge.ex_score(),
            judge,
            max_combo: MaxCombo::from_combo(self.combo),
            play_count: PlayCount::new(self.play_count),
            clear_count: ClearCount::new(self.clear_count),
            min_bp: MinBP::from_bp(self.min_bp),
            log: SnapShots::default(),
        }
    }

    pub fn get_score_id(&self) -> model::ScoreId {
        ScoreId::new(
            HashSha256::from_str(&self.sha256).unwrap(),
            PlayMode::from(self.mode),
        )
    }
}

impl CanGetHash for Score {
    fn hash_sha256(&self) -> String {
        self.sha256.clone()
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = scores)]
pub struct RegisteredScore {
    pub user_id: i32,
    pub sha256: String,
    pub mode: i32,
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
    pub combo: i32,
    pub min_bp: i32,
    pub play_count: i32,
    pub clear_count: i32,
    pub date: NaiveDateTime,
    pub score_upload_log_id: Option<i32>,
}

impl RegisteredScore {
    pub fn from_score(
        user_id: i32,
        score: &model::Score,
        song_id: &ScoreId,
        upload_id: &UploadId,
    ) -> Self {
        RegisteredScore {
            user_id,
            sha256: song_id.sha256().to_string(),
            mode: song_id.mode().to_int(),
            clear: score.clear.to_integer(),
            epg: score.judge.early_pgreat,
            lpg: score.judge.late_pgreat,
            egr: score.judge.early_great,
            lgr: score.judge.late_great,
            egd: score.judge.early_good,
            lgd: score.judge.late_good,
            ebd: score.judge.early_bad,
            lbd: score.judge.late_bad,
            epr: score.judge.early_poor,
            lpr: score.judge.late_poor,
            ems: score.judge.early_miss,
            lms: score.judge.late_miss,
            combo: score.max_combo.0,
            min_bp: score.min_bp.0,
            play_count: score.play_count.0,
            clear_count: 0,
            date: score.updated_at.naive_datetime(),
            score_upload_log_id: Some(upload_id.0),
        }
    }
}

impl CanGetHash for RegisteredScore {
    fn hash_sha256(&self) -> String {
        self.sha256.clone()
    }
}

pub trait CanGetHash {
    fn hash_sha256(&self) -> String;
}
