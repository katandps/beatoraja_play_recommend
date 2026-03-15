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
