table! {
    song(sha256, path) {
        md5 -> Text,
        sha256 -> Text,
        title -> Text,
        path -> Text,
        subtitle -> Text,
        subartist -> Text,
        artist -> Text,
        notes -> Integer,
        maxbpm -> Integer,
        minbpm -> Integer,
        length -> Integer,
    }
}

#[derive(Queryable)]
pub struct Song {
    pub md5: String,
    pub sha256: String,
    pub title: String,
    pub path: String,
    pub subtitle: String,
    pub subartist: String,
    pub artist: String,
    pub notes: i32,
    pub maxbpm: i32,
    pub minbpm: i32,
    pub length: i32,
}
