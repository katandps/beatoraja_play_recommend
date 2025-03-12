use anyhow::Result;
use chrono::Duration;

use crate::*;
use std::{collections::HashMap, str::FromStr};

#[derive(Clone)]
pub struct Scores(HashMap<ScoreId, Score>);

impl Scores {
    pub fn create_by_map(scores: HashMap<ScoreId, Score>) -> Self {
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

    pub fn table_scores<'a>(
        mut self,
        tables: &'a Tables,
        songs: &'a Songs,
        date: &'a UpdatedAt,
        account: &'a Account,
    ) -> DetailResponse {
        DetailResponse {
            user_id: account.user_id(),
            user_name: account.user_name(),
            score: tables
                .get_charts()
                .filter_map(|chart| {
                    let score_id = songs
                        .song(chart)
                        .map(|song| song.song_id())
                        .unwrap_or_default();
                    self.0
                        .remove(&score_id)
                        .map(|score| (chart.md5().clone(), score.make_detail(date)))
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DetailResponse {
    user_id: UserId,
    user_name: String,
    score: HashMap<HashMd5, ScoreDetail>,
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct DetailQuery {
    pub date: UpdatedAt,
    #[serde(default)]
    pub play_mode: PlayMode,
}

impl DetailQuery {
    pub async fn parse(query: HashMap<String, String>) -> Result<Self> {
        let date = query
            .get("date")
            .map(|u| {
                UpdatedAt::from_str(u)
                    .map(|u| &u - Duration::days(-1))
                    .unwrap_or_else(|_| UpdatedAt::default())
            })
            .unwrap_or_default();
        let play_mode = if let Some(mode) = query.get("mode") {
            match mode.parse::<i32>() {
                Ok(mode) => PlayMode::from(mode),
                Err(_) => PlayMode::default(),
            }
        } else {
            PlayMode::default()
        };
        Ok(DetailQuery { date, play_mode })
    }
}
