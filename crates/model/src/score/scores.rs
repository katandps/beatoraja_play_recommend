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
        period: &SnapPeriod,
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
                        .map(|score| {
                            score
                                .make_detail(period)
                                .map(|detail| (chart.md5().clone(), detail))
                        })
                        .flatten()
                })
                .collect(),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[allow(unused)]
pub struct DetailQuery {
    pub user_id: UserId,
    #[serde(flatten)]
    pub period: SnapPeriod,
    #[serde(default)]
    pub play_mode: PlayMode,
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn detail_query() {
        let json = r#"{
          "since": "2025-03-14T00:00:00Z",
          "until": "2025-03-16T00:00:00Z",
          "user_id": 9
        }"#;
        let q: DetailQuery = serde_json::from_str(json).unwrap();
        assert_eq!(
            q,
            DetailQuery {
                user_id: UserId::new(9),
                period: SnapPeriod {
                    since: DateTime::parse_from_rfc3339("2025-03-14T00:00:00+00:00")
                        .unwrap()
                        .into(),
                    until: DateTime::parse_from_rfc3339("2025-03-16T00:00:00+00:00")
                        .unwrap()
                        .into(),
                },
                play_mode: PlayMode::from(0)
            }
        )
    }
}
