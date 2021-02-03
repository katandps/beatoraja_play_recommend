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

    pub fn out<'a>(
        &self,
        tables: &'a Tables,
        songs: &'a Songs,
        date: &'a UpdatedAt,
        account: &'a Account,
    ) -> DetailResponse<'a> {
        let mut map = HashMap::new();
        let charts = tables.get_charts();
        for chart in &charts {
            let song = songs.song(chart);
            map.insert(
                chart.md5(),
                self.get(&song.song_id()).map(|s| s.make_detail(date)),
            );
        }
        DetailResponse {
            user_id: account.user_id(),
            user_name: account.user_name(),
            score: map,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DetailResponse<'a> {
    user_id: i32,
    user_name: String,
    score: HashMap<&'a HashMd5, Option<ScoreDetail>>,
}
