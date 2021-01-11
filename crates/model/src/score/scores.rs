use crate::*;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Scores(HashMap<ScoreId, Score>);

impl Scores {
    pub fn new(scores: HashMap<ScoreId, Score>) -> Scores {
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

    pub fn filter(&self, ids: &Vec<ScoreId>) -> HashMap<ScoreId, &Score> {
        let mut ret = HashMap::new();
        for id in ids {
            if let Some(score) = self.get(id) {
                ret.insert(id.clone(), score);
            }
        }
        ret
    }

    /// Tableに存在する曲ログに絞り込む ログが存在しない曲は未プレイとして作成される
    fn filter_by_table(&self, table: &Table, songs: &Songs) -> Self {
        let song_ids: Vec<ScoreId> = table
            .get_song(songs)
            .iter()
            .map(|song| song.song_id())
            .collect();
        let mut map = HashMap::new();
        for song_id in &song_ids {
            map.insert(
                song_id.clone(),
                self.get(&song_id).cloned().unwrap_or(Score::default()),
            );
        }
        Scores(map)
    }

    pub fn detail(
        &self,
        table: &Table,
        songs: &Songs,
        date: &UpdatedAt,
        level: Level,
    ) -> Vec<SongDetail> {
        self.filter_by_table(table, songs)
            .0
            .iter()
            .map(|(id, score)| {
                SongDetail::new(songs.song_by_id(id), score.clone(), date, level.clone())
            })
            .sorted_by(SongDetail::cmp_title)
            .collect()
    }
}

use anyhow::Result;
pub trait ScoreRepository {
    fn score(&self) -> Scores;
    fn save_score(&self, _account: Account, _score: Scores) -> Result<()>;
}
