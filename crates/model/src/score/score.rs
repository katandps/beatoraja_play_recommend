use crate::*;

#[derive(Clone, Debug)]
pub struct Score {
    pub clear: ClearType,
    pub updated_at: UpdatedAt,
    judge: Judge,
    pub score: ExScore,
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
        let score = judge.ex_score();
        Score {
            clear,
            updated_at,
            judge,
            score,
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

    pub fn at(self, date: &UpdatedAt) -> Score {
        if date.is_today() {
            self
        } else {
            let snap = self.log.get_snap(date);
            Score {
                clear: snap.clear_type,
                updated_at: snap.updated_at,
                judge: Judge::default(),
                score: snap.score,
                max_combo: snap.max_combo,
                play_count: PlayCount::new(-1),
                min_bp: snap.min_bp,
                log: SnapShots::default(),
            }
        }
    }
}
