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
        date: &UpdatedAt,
        sha256: &HashSha256,
        users: &[VisibleAccount],
    ) -> Option<RankingResponse> {
        songs.song_by_sha256(sha256).map(|song| RankingResponse {
            song: song.into(),
            score: users
                .iter()
                .filter_map(|va| {
                    self.0
                        .remove(&va.id)
                        .map(|score| (va.id, (va.name.clone(), score.make_detail(date))))
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct RankingResponse {
    song: SongFormat,
    score: HashMap<UserId, (String, ScoreDetail)>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct RankingQuery {
    pub date: UpdatedAt,
    #[serde(default)]
    pub play_mode: PlayMode,
    pub sha256: HashSha256,
}
