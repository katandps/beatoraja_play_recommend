use super::*;

pub struct SnapShots {
    pub(super) song_id: SongId,
    pub(super) snapshots: Vec<SnapShot>,
}

impl SnapShots {
    pub(super) fn add(&mut self, snapshot: SnapShot) {
        self.snapshots.push(snapshot)
    }

    pub(super) fn get_snap(&self, date: &UpdatedAt) -> SnapShot {
        let snap = self
            .snapshots
            .iter()
            .filter(|s| s.updated_at.le(date))
            .map(|s| s.clone())
            .last();
        match snap {
            Some(s) => s,
            _ => SnapShot::new(self.song_id.clone()),
        }
    }
}
