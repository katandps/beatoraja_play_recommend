use crate::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

/// 最新スコアのみが入っている
#[derive(Clone)]
pub struct Scores(HashMap<SongId, Score>);

impl Scores {
    pub fn new(scores: HashMap<SongId, Score>) -> Scores {
        Scores(scores)
    }
    pub fn count(&self) -> usize {
        self.0.len()
    }
    pub fn get(&self, song_id: &SongId) -> Option<&Score> {
        self.0.get(song_id)
    }

    /// Tableに存在する曲ログに絞り込む ログが存在しない曲はダミーで補完される
    fn filter_by_table<T: TableTrait>(&self, table: &T, songs: &Songs) -> Self {
        let song_ids: Vec<SongId> = table
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

    pub fn detail<T: TableTrait>(
        &self,
        table: &T,
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
            .sorted_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()))
            .collect()
    }
}

impl fmt::Display for Scores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for (song_id, score) in &self.0 {
            result.push_str(&format!("{}: {}\n", song_id, score.view()));
        }
        write!(f, "{}", result)
    }
}

pub trait ScoreRepository {
    fn score(&self) -> Scores;
}
