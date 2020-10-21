use crate::*;

#[derive(Clone, Debug)]
pub struct Score {
    pub clear: ClearType,
    pub updated_at: UpdatedAt,
    pub judge: Judge,
    pub max_combo: MaxCombo,
    pub play_count: PlayCount,
    pub min_bp: MinBP,
    log: SnapShots,
}

impl Score {
    pub fn new(
        clear: ClearType,
        updated_at: UpdatedAt,
        judge: Judge,
        max_combo: MaxCombo,
        play_count: PlayCount,
        min_bp: MinBP,
        log: SnapShots,
    ) -> Score {
        Score {
            clear,
            updated_at,
            judge,
            max_combo,
            play_count,
            min_bp,
            log,
        }
    }

    pub fn default() -> Score {
        Score::new(
            ClearType::NoPlay,
            UpdatedAt::new(),
            Judge::default(),
            MaxCombo::new(),
            PlayCount::new(0),
            MinBP::new(),
            SnapShots::default(),
        )
    }

    pub fn view(&self) -> String {
        format!(
            "{} {} score:{} bp:{} combo:{}",
            self.updated_at,
            self.clear,
            self.judge.ex_score(),
            self.min_bp,
            self.max_combo
        )
    }
}
