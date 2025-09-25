mod clear_count;
mod clear_type;
mod ex_score;
mod judge;
mod max_combo;
mod min_bp;
mod play_count;
mod rank;
mod ranking;
mod scores;
mod snap_range;
mod snapshot;
mod snapshots;
mod song_id;
mod updated_at;

pub use {
    clear_count::ClearCount,
    clear_type::ClearType,
    ex_score::ExScore,
    judge::Judge,
    max_combo::MaxCombo,
    min_bp::MinBP,
    play_count::PlayCount,
    rank::ClearRank,
    ranking::{RankedScore, RankingQuery, RankingResponse},
    scores::{DetailQuery, DetailResponse, Scores},
    snap_range::SnapPeriod,
    snapshot::SnapShot,
    snapshots::SnapShots,
    song_id::{PlayMode, ScoreId},
    updated_at::UpdatedAt,
};

use crate::*;

#[derive(Clone, Debug, Default, Serialize)]
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
    pub fn with_log(self, log: SnapShots) -> Self {
        Score { log, ..self }
    }

    pub fn snap(&self, period: &SnapPeriod) -> Option<&SnapShot> {
        self.log.snap(period)
    }

    pub fn param_snap<T: ParamSnap>(&self, period: &SnapPeriod) -> Option<T> {
        self.log.param_snap::<T>(period)
    }

    pub fn make_detail(self, period: &SnapPeriod) -> Option<ScoreDetail> {
        match self.snap(period) {
            Some(snap) => Some(ScoreDetail {
                clear_type: self.param_snap(period),
                min_bp: self.param_snap(period),
                score: self.param_snap(period),
                max_combo: snap.max_combo.clone(),
                updated_at: snap.updated_at.clone(),
                play_count: if period.is_past_range() {
                    PlayCount::new(-1)
                } else {
                    self.play_count.clone()
                },
            }),
            None if !self.log.has_snap() => {
                // imported but no play
                if period.contains(&self.updated_at) {
                    Some(ScoreDetail {
                        max_combo: self.max_combo,
                        score: Some(ScoreSnap::from(self.score)),
                        min_bp: Some(MinBPSnap::from(self.min_bp)),
                        clear_type: Some(ClearTypeSnap::from(self.clear)),
                        updated_at: self.updated_at,
                        play_count: self.play_count,
                    })
                } else {
                    None
                }
            }
            None => None,
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

impl From<ExScore> for ScoreSnap {
    fn from(value: ExScore) -> Self {
        ScoreSnap {
            current: value,
            ..Default::default()
        }
    }
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

impl From<MinBP> for MinBPSnap {
    fn from(value: MinBP) -> Self {
        MinBPSnap {
            current: value,
            ..Default::default()
        }
    }
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
    pub current: i32,
    pub updated_at: UpdatedAt,
    pub before: i32,
}

impl From<ClearType> for ClearTypeSnap {
    fn from(value: ClearType) -> Self {
        ClearTypeSnap {
            current: value.to_integer(),
            ..Default::default()
        }
    }
}

impl ParamSnap for ClearTypeSnap {
    fn make(current: &SnapShot, updated_at: UpdatedAt, before_snap: Option<&SnapShot>) -> Self {
        ClearTypeSnap {
            current: current.clear_type.to_integer(),
            updated_at,
            before: match before_snap {
                Some(s) => s.clear_type.to_integer(),
                None => ClearType::default().to_integer(),
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

#[derive(Deserialize)]
pub struct SongLogQuery {
    pub user_id: UserId,
    #[serde(default)]
    pub play_mode: PlayMode,
    pub sha256: HashSha256,
}

#[derive(Deserialize)]
pub struct SongMyLogQuery {
    #[serde(default)]
    pub play_mode: PlayMode,
    pub sha256: HashSha256,
}
