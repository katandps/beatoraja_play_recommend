use crate::*;
use std::collections::HashMap;

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
    pub fn remove(&mut self, song_id: &ScoreId) -> Option<Score> {
        self.0.remove(song_id)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DetailResponse {
    user_id: UserId,
    user_name: String,
    score: HashMap<HashMd5, ScoreDetail>,
}

impl DetailResponse {
    pub fn new(
        tables: &Tables,
        songs: &Songs,
        mut scores: Scores,
        date: &UpdatedAt,
        after_date: &UpdatedAt,
        account: &Account,
    ) -> Self {
        Self {
            user_id: account.user_id(),
            user_name: account.user_name(),
            score: tables
                .get_charts()
                .filter_map(|chart| {
                    let score_id = songs
                        .song(chart)
                        .map(|song| song.song_id())
                        .unwrap_or_default();
                    scores
                        .remove(&score_id)
                        .filter(|score| &score.updated_at >= after_date)
                        .map(|score| (chart.md5().clone(), score.make_detail(date)))
                })
                .collect(),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[allow(unused)]
pub struct DetailQuery {
    pub user_id: UserId,
    pub date: UpdatedAt,
    #[serde(default)]
    pub after_date: UpdatedAt,
    #[serde(default)]
    pub play_mode: PlayMode,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn detail_query() {
        let json = r#"{
          "date": "2025-03-14T00:00:00Z",
          "user_id": 9
        }"#;
        let q: DetailQuery = serde_json::from_str(json).unwrap();
        assert_eq!(
            q,
            DetailQuery {
                user_id: UserId::new(9),
                date: UpdatedAt::from_str("2025-03-14").unwrap(),
                after_date: UpdatedAt::from_timestamp(0),
                play_mode: PlayMode::from(0)
            }
        )
    }
}
