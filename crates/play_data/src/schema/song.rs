table! {
    song(sha256, path) {
        sha256 -> Text,
        path -> Text,
        md5 -> Text,
        title -> Text,
        subtitle -> Text,
        artist -> Text,
        notes -> Integer,
    }
}

#[derive(Queryable)]
pub struct Song {
    pub sha256: String,
    pub path: String,
    pub md5: String,
    pub title: String,
    pub subtitle: String,
    pub artist: String,
    pub notes: i32,
}
