use crate::*;

pub struct SongWithSnap<'a> {
    song: &'a Song,
    snap: SnapShot,
}

impl<'a> SongWithSnap<'a> {
    pub fn make(song: &'a Song, snap: SnapShot) -> SongWithSnap<'a> {
        SongWithSnap { song, snap }
    }

    pub fn clear_rank(&self) -> ClearRank {
        ClearRank::from_notes_score(self.song.notes, self.snap.score())
    }
}
