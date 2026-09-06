use crate::{PlayerStat, PlayerStatDiff};
use crate::{UserId, UserName};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct ScoreUpload {
    pub upload_id: UploadId,
    pub upload_at: UploadAt,
    pub song_count: i64,
    pub stats: PlayerStatDiff,
    pub total_stats: PlayerStat,
}

impl ScoreUpload {
    pub fn new(upload_id: UploadId, upload_at: UploadAt) -> Self {
        Self::with_stats(
            upload_id,
            upload_at,
            0,
            PlayerStatDiff::default(),
            PlayerStat::default(),
        )
    }

    pub fn with_stats(
        upload_id: UploadId,
        upload_at: UploadAt,
        song_count: i64,
        stats: PlayerStatDiff,
        total_stats: PlayerStat,
    ) -> Self {
        ScoreUpload {
            upload_id,
            upload_at,
            song_count,
            stats,
            total_stats,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UploadId(pub i32);

impl UploadId {
    pub fn get(&self) -> i32 {
        self.0
    }

    pub fn from_integer(id: i32) -> Self {
        UploadId(id)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UploadAt(pub DateTime<Utc>);

impl UploadAt {
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }
}

pub struct ScoreUploadInfo {
    pub upload_id: UploadId,
    pub user_id: UserId,
    pub user_name: UserName,
    pub stat: PlayerStatDiff,
}

impl ScoreUploadInfo {
    pub fn new(
        upload_id: UploadId,
        user_id: UserId,
        user_name: UserName,
        stat: PlayerStatDiff,
    ) -> Self {
        Self {
            upload_id,
            user_id,
            user_name,
            stat,
        }
    }
}
