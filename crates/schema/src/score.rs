use schemars::JsonSchema;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct ScoreUploadInfo {
    upload_id: UploadID,
    user_id: UserId,
    user_name: UserName,
    stats: PlayerStatDiff,
    score: DetailScore,
}

impl ScoreUploadInfo {
    pub fn new(upload: model::ScoreUploadInfo, scores: model::DetailScore) -> Self {
        Self {
            upload_id: upload.upload_id.get(),
            user_id: upload.user_id.get(),
            user_name: upload.user_name.to_string(),
            stats: PlayerStatDiff::new(upload.stat),
            score: DetailScore::new(scores),
        }
    }
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct PlayerStatDiff {
    before_date: PlayedAt,
    after_date: PlayedAt,
    play_count: PlayCount,
    clear_count: ClearCount,
    play_time: PlayTime,
    total_judge: Judge,
}

impl PlayerStatDiff {
    pub fn new(stats: model::PlayerStatDiff) -> Self {
        Self {
            before_date: stats.before_date().to_rfc3339(),
            after_date: stats.after_date().to_rfc3339(),
            play_count: stats.play_count().0,
            clear_count: stats.clear_count().0,
            play_time: stats.play_time().0,
            total_judge: Judge::new(stats.total_judge().judge()),
        }
    }
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct Judge {
    early_pgreat: JudgeCount,
    late_pgreat: JudgeCount,
    early_great: JudgeCount,
    late_great: JudgeCount,
    early_good: JudgeCount,
    late_good: JudgeCount,
    early_bad: JudgeCount,
    late_bad: JudgeCount,
    early_poor: JudgeCount,
    late_poor: JudgeCount,
    early_miss: JudgeCount,
    late_miss: JudgeCount,
}

impl Judge {
    pub fn new(judge: &model::Judge) -> Self {
        Self {
            early_pgreat: judge.early_pgreat,
            late_pgreat: judge.late_pgreat,
            early_great: judge.early_great,
            late_great: judge.late_great,
            early_good: judge.early_good,
            late_good: judge.late_good,
            early_bad: judge.early_bad,
            late_bad: judge.late_bad,
            early_poor: judge.early_poor,
            late_poor: judge.late_poor,
            early_miss: judge.early_miss,
            late_miss: judge.late_miss,
        }
    }
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct ScoreDetail {
    max_combo: MaxCombo,
    score: Option<ScoreSnap>,
    min_bp: Option<MinBPSnap>,
    clear_type: Option<ClearTypeSnap>,
    updated_at: UpdatedAt,
    play_count: PlayCount,
}

impl ScoreDetail {
    pub fn new(detail: model::ScoreDetail) -> Self {
        Self {
            max_combo: detail.max_combo().0,
            score: detail.score().cloned().map(ScoreSnap::new),
            min_bp: detail.min_bp().cloned().map(MinBPSnap::new),
            clear_type: detail.clear_type().cloned().map(ClearTypeSnap::new),
            updated_at: detail.updated_at().to_rfc3339(),
            play_count: detail.play_count().0,
        }
    }
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct MinBPSnap {
    current: MinBP,
    updated_at: UpdatedAt,
    before: MinBP,
}

impl MinBPSnap {
    pub fn new(snap: model::MinBPSnap) -> Self {
        Self {
            current: snap.current.0,
            updated_at: snap.updated_at.to_rfc3339(),
            before: snap.before.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct ClearTypeSnap {
    current: ClearCount,
    updated_at: UpdatedAt,
    before: ClearCount,
}

impl ClearTypeSnap {
    pub fn new(snap: model::ClearTypeSnap) -> Self {
        Self {
            current: snap.current,
            updated_at: snap.updated_at.to_rfc3339(),
            before: snap.before,
        }
    }
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct ScoreSnap {
    current: ExScore,
    updated_at: UpdatedAt,
    before: ExScore,
}

impl ScoreSnap {
    pub fn new(snap: model::ScoreSnap) -> Self {
        Self {
            current: snap.current.ex_score(),
            updated_at: snap.updated_at.to_rfc3339(),
            before: snap.before.ex_score(),
        }
    }
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct DetailScore(HashMap<String, ScoreDetail>);

impl DetailScore {
    fn new(scores: model::DetailScore) -> Self {
        Self(
            scores
                .0
                .into_iter()
                .map(|(k, v)| (k.to_string(), ScoreDetail::new(v)))
                .collect(),
        )
    }
}

pub type UpdatedAt = String;
pub type PlayCount = i32;
pub type ClearCount = i32;
pub type MaxCombo = i32;
pub type MinBP = i32;
pub type UploadID = i32;
pub type UserId = i32;
pub type UserName = String;
pub type PlayTime = i32;
pub type JudgeCount = i32;
pub type ExScore = i32;
pub type UploadAt = String;
pub type PlayedAt = String;
