use crate::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ScoreLog(HashMap<SongId, SnapShots>);

impl ScoreLog {
    pub fn get_snaps(&self, song_id: &SongId) -> SnapShots {
        self.0.get(song_id).unwrap().clone()
    }
}

pub struct ScoreLogBuilder(HashMap<SongId, SnapShots>);

impl ScoreLogBuilder {
    pub fn push(&mut self, song_id: SongId, snapshot: SnapShot) {
        self.0
            .entry(song_id.clone())
            .or_insert(SnapShots::default())
            .add(snapshot);
    }

    pub fn builder() -> ScoreLogBuilder {
        ScoreLogBuilder(HashMap::new())
    }

    pub fn build(self) -> ScoreLog {
        ScoreLog(self.0)
    }
}

pub trait ScoreLogRepository {
    fn score_log(&self) -> ScoreLog;
}
