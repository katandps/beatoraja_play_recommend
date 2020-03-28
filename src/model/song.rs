#[derive(Queryable)]
pub struct Song {
    pub sha256: String,
    pub path: String,
    pub md5: String,
    pub title: String,
    pub subtitle: String,
}