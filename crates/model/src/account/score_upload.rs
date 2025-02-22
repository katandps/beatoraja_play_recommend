use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ScoreUpload {
    pub upload_id: UploadId,
    pub upload_at: UploadAt,
}

impl ScoreUpload {
    pub fn new(upload_id: UploadId, upload_at: UploadAt) -> Self {
        ScoreUpload {
            upload_id,
            upload_at,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UploadId(pub i32);

#[derive(Clone, Debug, Serialize)]
pub struct UploadAt(pub DateTime<Utc>);
