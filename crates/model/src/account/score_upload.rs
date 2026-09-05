use crate::{PlayerStat, PlayerStatDiff};
use chrono::{DateTime, Utc};
use serde::Serialize;

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

#[derive(Clone, Debug, Serialize)]
pub struct UploadId(pub i32);

impl UploadId {
    pub fn get(&self) -> i32 {
        self.0
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UploadAt(pub DateTime<Utc>);

impl UploadAt {
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }
}
