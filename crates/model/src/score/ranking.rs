use crate::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct RankedScore(pub HashMap<UserId, Score>);

impl RankedScore {
    pub fn create_by_map(scores: HashMap<UserId, Score>) -> Self {
        RankedScore(scores)
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct RankingQuery {
    #[serde(flatten)]
    pub date: SnapPeriod,
    #[serde(default)]
    pub play_mode: PlayMode,
    pub sha256: HashSha256,
}
