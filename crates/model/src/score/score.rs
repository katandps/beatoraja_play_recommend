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

    pub fn param_snap<T: ParamSnap>(&self, date: &UpdatedAt) -> Option<T> {
        self.log.param_snap::<T>(date)
    }

    pub fn make_detail(&self, date: &UpdatedAt) -> ScoreDetail {
        match self.snap(date) {
            Some(snap) => ScoreDetail {
                clear_type: self.param_snap(date),
                min_bp: self.param_snap(date),
                score: self.param_snap(date),
                max_combo: snap.max_combo.clone(),
                updated_at: snap.updated_at.clone(),
                play_count: if !date.is_future() {
                    PlayCount::new(-1)
                } else {
                    self.play_count.clone()
                },
            },
            None => Default::default(),
        }
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

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ScoreSnap {
    pub current: ExScore,
    pub updated_at: UpdatedAt,
    pub before: ExScore,
}

impl ParamSnap for ScoreSnap {
    fn make(current: &SnapShot, updated_at: UpdatedAt, before_snap: Option<&SnapShot>) -> Self {
        ScoreSnap {
            current: current.score,
            updated_at,
            before: match before_snap {
                Some(s) => s.score,
                None => Default::default(),
            },
        }
    }
}

impl SnapCmp for ScoreSnap {
    fn cmp(a: &SnapShot, b: &SnapShot) -> bool {
        a.score >= b.score
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct MinBPSnap {
    pub current: MinBP,
    pub updated_at: UpdatedAt,
    pub before: MinBP,
}

impl ParamSnap for MinBPSnap {
    fn make(current: &SnapShot, updated_at: UpdatedAt, before_snap: Option<&SnapShot>) -> Self {
        MinBPSnap {
            current: current.min_bp,
            updated_at,
            before: match before_snap {
                Some(s) => s.min_bp,
                None => Default::default(),
            },
        }
    }
}

impl SnapCmp for MinBPSnap {
    fn cmp(a: &SnapShot, b: &SnapShot) -> bool {
        a.min_bp <= b.min_bp
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ClearTypeSnap {
    pub current: ClearType,
    pub updated_at: UpdatedAt,
    pub before: ClearType,
}

impl ParamSnap for ClearTypeSnap {
    fn make(current: &SnapShot, updated_at: UpdatedAt, before_snap: Option<&SnapShot>) -> Self {
        ClearTypeSnap {
            current: current.clear_type,
            updated_at,
            before: match before_snap {
                Some(s) => s.clear_type,
                None => Default::default(),
            },
        }
    }
}

impl SnapCmp for ClearTypeSnap {
    fn cmp(a: &SnapShot, b: &SnapShot) -> bool {
        a.clear_type >= b.clear_type
    }
}

pub trait SnapCmp {
    fn cmp(a: &SnapShot, b: &SnapShot) -> bool;
}
pub trait ParamSnap: SnapCmp {
    fn make(current: &SnapShot, updated_at: UpdatedAt, before_snap: Option<&SnapShot>) -> Self;
}
