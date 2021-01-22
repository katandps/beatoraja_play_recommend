use crate::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Scores(HashMap<ScoreId, Score>);

impl Scores {
    pub fn create_by_map(scores: HashMap<ScoreId, Score>) -> Scores {
        Scores(scores)
    }
    pub fn count(&self) -> usize {
        self.0.len()
    }
    pub fn get(&self, song_id: &ScoreId) -> Option<&Score> {
        self.0.get(song_id)
    }
    pub fn get_map(&self) -> &HashMap<ScoreId, Score> {
        &self.0
    }

    pub fn out(
        &self,
        tables: &Tables,
        songs: &Songs,
        date: UpdatedAt,
    ) -> HashMap<HashMd5, SongDetail> {
        let mut map = HashMap::new();
        let charts = tables.get_charts();
        for chart in &charts {
            let song = songs.song(chart);
            let score = match self.get(&song.song_id()) {
                Some(s) => s.clone(),
                None => Score::default(),
            };
            map.insert(chart.md5(), SongDetail::new(&song, &score, &date));
        }
        map
    }
}
