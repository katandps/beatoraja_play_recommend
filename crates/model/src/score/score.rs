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

    pub fn snap(&self, date: &UpdatedAt) -> Option<&SnapShot> {
        self.log.snap(date)
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

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ScoreDetail {
    max_combo: MaxCombo,
    score: Option<ScoreSnap>,
    min_bp: Option<MinBPSnap>,
    clear_type: Option<ClearTypeSnap>,
    updated_at: UpdatedAt,
    play_count: PlayCount,
}

impl ScoreDetail {
    pub fn new(score: &Score, date: &UpdatedAt) -> ScoreDetail {
        match score.snap(date) {
            Some(snap) => ScoreDetail {
                clear_type: score.clear_type_snap(date),
                min_bp: score.min_bp_snap(date),
                score: score.score_snap(date),
                max_combo: snap.max_combo.clone(),
                updated_at: snap.updated_at.clone(),
                play_count: if date.is_future() {
                    PlayCount::new(-1)
                } else {
                    score.play_count.clone()
                },
            },
            None => Default::default(),
        }
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
