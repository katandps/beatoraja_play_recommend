use chrono::NaiveDateTime;
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
pub struct UploadId(usize);

#[derive(Clone, Debug, Serialize)]
pub struct UploadAt(NaiveDateTime);
