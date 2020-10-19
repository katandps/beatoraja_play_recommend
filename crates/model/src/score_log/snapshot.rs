use crate::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SnapShot {
    pub clear_type: ClearType,
    pub score: ExScore,
    pub max_combo: MaxCombo,
    pub min_bp: MinBP,
    pub updated_at: UpdatedAt,
}

impl SnapShot {
    pub fn new() -> SnapShot {
        SnapShot {
            clear_type: ClearType::NoPlay,
            score: ExScore::new(),
            max_combo: MaxCombo::new(),
            min_bp: MinBP::new(),
            updated_at: UpdatedAt::new(),
        }
    }
    pub fn from_data(
        clear_type: i32,
        score: i32,
        combo: i32,
        minbp: i32,
        timestamp: i32,
    ) -> SnapShot {
        SnapShot {
            clear_type: ClearType::from_integer(clear_type),
            score: ExScore::from_score(score),
            max_combo: MaxCombo::from_combo(combo),
            min_bp: MinBP::from_bp(minbp),
            updated_at: UpdatedAt::from_timestamp(timestamp),
        }
    }

    pub fn score(&self) -> ExScore {
        self.score.clone()
    }
    pub fn clear_type(&self) -> &ClearType {
        &self.clear_type
    }
}
