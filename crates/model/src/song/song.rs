use crate::song::include_features::IncludeFeatures;
use crate::*;

#[derive(Clone, Debug)]
pub struct Song {
    hash: HashSha256,
    title: Title,
    artist: Artist,
    notes: i32,
    include_features: IncludeFeatures,
}

impl Song {
    pub fn new(
        hash: HashSha256,
        title: Title,
        artist: Artist,
        notes: i32,
        include_features: IncludeFeatures,
    ) -> Song {
        Song {
            hash,
            title,
            artist,
            notes,
            include_features,
        }
    }

    pub fn song_id(&self) -> ScoreId {
        ScoreId::new(self.hash.clone(), PlayMode::new(0))
    }

    pub fn title(&self) -> String {
        self.title.to_string()
    }
    pub fn artist(&self) -> String {
        self.artist.to_string()
    }
    pub fn notes(&self) -> i32 {
        self.notes
    }
    pub fn get_hash(&self) -> &HashSha256 {
        &self.hash
    }
    pub fn features(&self) -> &IncludeFeatures {
        &self.include_features
    }
}
