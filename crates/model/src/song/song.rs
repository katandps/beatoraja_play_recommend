use crate::*;

#[derive(Clone, Debug)]
pub struct Song {
    pub hash: HashSha256,
    pub title: Title,
    pub artist: Artist,
    pub notes: i32,
}

impl Song {
    pub fn song_id(&self) -> SongId {
        SongId::new(self.hash.clone(), PlayMode::new(0))
    }

    pub fn title(&self) -> String {
        self.title.to_string()
    }
    pub fn notes(&self) -> i32 {
        self.notes
    }
}
