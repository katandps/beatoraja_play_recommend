use crate::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct RankedScore(HashMap<UserId, Score>);

impl RankedScore {
    pub fn create_by_map(scores: HashMap<UserId, Score>) -> Self {
        RankedScore(scores)
    }

    pub fn for_response(
        &self,
        songs: &Songs,
        date: &UpdatedAt,
        sha256: &HashSha256,
    ) -> Option<RankingResponse> {
        match songs.song_by_sha256(sha256) {
            Some(song) => {
                let score = self
                    .0
                    .iter()
                    .map(|(user_id, score)| (user_id.clone(), score.make_detail(date)))
                    .collect();
                Some(RankingResponse {
                    song: song.into(),
                    score,
                })
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct RankingResponse {
    song: SongFormat,
    score: HashMap<UserId, ScoreDetail>,
}
