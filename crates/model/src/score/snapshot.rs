use crate::*;
use std::cmp::Ordering;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct SnapShot {
    pub clear_type: ClearType,
    pub score: ExScore,
    pub max_combo: MaxCombo,
    pub min_bp: MinBP,
    pub updated_at: UpdatedAt,
}

impl SnapShot {
    pub fn from_data(
        clear_type: i32,
        score: i32,
        combo: i32,
        minbp: i32,
        timestamp: i64,
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
        self.score
    }
    pub fn clear_type(&self) -> &ClearType {
        &self.clear_type
    }
}

impl PartialEq for SnapShot {
    fn eq(&self, other: &Self) -> bool {
        self.updated_at == other.updated_at
    }
}

impl Eq for SnapShot {}

impl PartialOrd for SnapShot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SnapShot {
    fn cmp(&self, other: &Self) -> Ordering {
        self.updated_at.cmp(&other.updated_at)
    }
}
