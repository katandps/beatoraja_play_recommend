use crate::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct RankedScore(HashMap<UserId, Score>);

impl RankedScore {
    pub fn create_by_map(scores: HashMap<UserId, Score>) -> Self {
        RankedScore(scores)
    }

    pub fn for_response(
        mut self,
        songs: &Songs,
        date: &SnapPeriod,
        sha256: &HashSha256,
        users: &[VisibleAccount],
    ) -> Option<RankingResponse> {
        songs.song_by_sha256(sha256).map(|song| RankingResponse {
            song: song.into(),
            score: users
                .iter()
                .filter_map(|va| {
                    self.0.remove(&va.id).and_then(|score| {
                        score.make_detail(date).map(|detail| RankingScore {
                            user_id: va.id,
                            user_name: va.name.clone(),
                            score: detail,
                        })
                    })
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct RankingResponse {
    song: SongFormat,
    score: Vec<RankingScore>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RankingScore {
    user_id: UserId,
    user_name: UserName,
    score: ScoreDetail,
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
