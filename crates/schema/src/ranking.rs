use crate::{
    score::{ScoreDetail, UserId, UserName},
    song::SongResponse,
};
use model::{HashSha256, RankedScore, SnapPeriod, Songs, VisibleAccount};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default, JsonSchema)]
pub struct RankingResponse {
    song: SongResponse,
    score: Vec<RankingScore>,
}

impl RankingResponse {
    pub fn from_models(
        mut ranked_score: RankedScore,
        songs: &Songs,
        date: &SnapPeriod,
        sha256: &HashSha256,
        users: &[VisibleAccount],
    ) -> Option<Self> {
        songs.song_by_sha256(sha256).map(|song| Self {
            song: SongResponse::from_song(song),
            score: users
                .iter()
                .filter_map(|va| {
                    ranked_score.0.remove(&va.id).and_then(|score| {
                        score.make_detail(date).map(|detail| RankingScore {
                            user_id: va.id.get(),
                            user_name: va.name.clone().to_string(),
                            score: ScoreDetail::new(detail),
                        })
                    })
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct RankingScore {
    user_id: UserId,
    user_name: UserName,
    score: ScoreDetail,
}
