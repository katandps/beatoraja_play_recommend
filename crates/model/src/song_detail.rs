use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ScoreDetail {
    clear_rank: ClearRank,
    max_combo: MaxCombo,
    score: Option<ScoreSnap>,
    min_bp: Option<MinBPSnap>,
    clear_type: Option<ClearTypeSnap>,
    updated_at: UpdatedAt,
    play_count: PlayCount,
}

impl ScoreDetail {
    pub fn new(song: &Song, score: &Score, date: &UpdatedAt) -> ScoreDetail {
        let score = score.clone().at(date).clone();
        ScoreDetail {
            clear_type: score.clear_type_snap(date),
            clear_rank: ClearRank::from_notes_score(song.notes(), score.score),
            max_combo: score.max_combo.clone(),
            min_bp: score.min_bp_snap(date),
            score: score.score_snap(date),
            updated_at: score.updated_at,
            play_count: score.play_count,
        }
    }
}
