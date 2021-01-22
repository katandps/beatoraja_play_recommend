use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Score {
    pub clear: ClearType,
    pub updated_at: UpdatedAt,
    pub judge: Judge,
    pub score: ExScore,
    pub max_combo: MaxCombo,
    pub play_count: PlayCount,
    pub clear_count: ClearCount,
    pub min_bp: MinBP,
    pub log: SnapShots,
}

impl Score {
    pub fn new(
        clear: ClearType,
        updated_at: UpdatedAt,
        judge: Judge,
        max_combo: MaxCombo,
        play_count: PlayCount,
        clear_count: ClearCount,
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
            clear_count,
        }
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
        if date.is_future() {
            self
        } else {
            let snap = self.log.get_snap(date);
            Score {
                clear: snap.clear_type,
                updated_at: snap.updated_at,
                judge: Default::default(),
                score: snap.score,
                max_combo: snap.max_combo,
                play_count: PlayCount::new(-1),
                min_bp: snap.min_bp,
                log: Default::default(),
                clear_count: ClearCount::new(-1),
            }
        }
    }

    pub fn score_snap(&self, date: &UpdatedAt) -> Option<ScoreSnap> {
        self.log.score_snap(date)
    }
    pub fn min_bp_snap(&self, date: &UpdatedAt) -> Option<MinBPSnap> {
        self.log.min_bp_snap(date)
    }
    pub fn clear_type_snap(&self, date: &UpdatedAt) -> Option<ClearTypeSnap> {
        self.log.clear_type_snap(date)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ScoreSnap {
    pub current: ExScore,
    pub updated_at: UpdatedAt,
    pub before: ExScore,
}

impl ScoreSnap {
    pub fn new(current: ExScore, updated_at: UpdatedAt, before: ExScore) -> Self {
        ScoreSnap {
            current,
            updated_at,
            before,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct MinBPSnap {
    pub current: MinBP,
    pub updated_at: UpdatedAt,
    pub before: MinBP,
}

impl MinBPSnap {
    pub fn new(current: MinBP, updated_at: UpdatedAt, before: MinBP) -> Self {
        MinBPSnap {
            current,
            updated_at,
            before,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ClearTypeSnap {
    pub current: ClearType,
    pub updated_at: UpdatedAt,
    pub before: ClearType,
}

impl ClearTypeSnap {
    pub fn new(current: ClearType, updated_at: UpdatedAt, before: ClearType) -> Self {
        ClearTypeSnap {
            current,
            updated_at,
            before,
        }
    }
}
