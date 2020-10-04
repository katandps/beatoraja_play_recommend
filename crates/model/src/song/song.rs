use crate::*;

#[derive(Clone)]
pub struct Song {
    pub(super) hash: HashSha256,
    pub(super) title: Title,
    pub(super) artist: Artist,
    pub(super) notes: i32,
}

impl Song {
    pub fn song_id(&self) -> SongId {
        SongId::new(self.hash.clone(), PlayMode::new(0))
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
}
