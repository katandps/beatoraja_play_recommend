pub mod clear_type;
pub mod ex_score;
pub mod judge;
pub mod max_combo;
pub mod min_bp;
pub mod play_count;
pub mod scores;
pub mod song_id;
pub mod updated_at;

use std::cmp::Ordering;
use std::fmt;

use crate::lamp::Lamp;
use crate::score::judge::Judge;
use crate::score::max_combo::MaxCombo;
use crate::score::min_bp::MinBP;
use crate::score::play_count::PlayCount;
use clear_type::ClearType;
use updated_at::UpdatedAt;

#[derive(Clone)]
pub struct Score {
    clear: ClearType,
    updated_at: UpdatedAt,
    judge: Judge,
    max_combo: MaxCombo,
    play_count: PlayCount,
    min_bp: MinBP,
}

impl Score {
    pub fn from_data(
        clear: ClearType,
        updated_at: UpdatedAt,
        judge: Judge,
        max_combo: MaxCombo,
        play_count: PlayCount,
        min_bp: MinBP,
    ) -> Score {
        Score {
            clear,
            updated_at,
            judge,
            max_combo,
            play_count,
            min_bp,
        }
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.updated_at.cmp(&other.updated_at)
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.updated_at == other.updated_at
    }
}

impl Eq for Score {}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} score:{} bp:{} combo:{}",
            self.updated_at,
            self.clear,
            self.judge.ex_score(),
            self.min_bp,
            self.max_combo
        )
    }
}

impl Lamp for Score {
    fn clear_type(&self) -> &ClearType {
        &self.clear
    }
}
