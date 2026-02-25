use model::Song;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct SongResponse {
    pub md5: String,
    pub sha256: String,
    pub title: String,
    pub artist: String,
    pub notes: i32,
    pub include_features: i32,
}

impl SongResponse {
    pub fn from_song(song: &Song) -> Self {
        Self {
            md5: song.md5.to_string(),
            sha256: song.sha256.to_string(),
            title: song.title.to_string(),
            artist: song.artist.to_string(),
            notes: song.notes,
            include_features: song.include_features.clone().into(),
        }
    }
}
