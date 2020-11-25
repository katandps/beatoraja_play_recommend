use crate::{Judge, PlayCount, UpdatedAt};
use serde::Serialize;

#[derive(Debug)]
pub struct PlayerStates {
    log: Vec<PlayerState>,
}

impl PlayerStates {
    pub fn new(log: Vec<PlayerState>) -> PlayerStates {
        PlayerStates { log }
    }

    pub fn last(&self) -> Option<&PlayerState> {
        self.log.iter().last()
    }

    pub fn diff(&self) -> Vec<PlayerStateDiff> {
        let mut log = self.log.clone();
        log.sort_by(|a, b| a.date.cmp(&b.date));
        let mut ret = Vec::new();
        for i in 1..log.len() {
            let before = log[i - 1].clone();
            let after = log[i].clone();
            ret.push(PlayerStateDiff::new(
                before.date,
                after.date,
                after.play_count - before.play_count,
                after.clear_count - before.clear_count,
                after.play_time - before.play_time,
                after.total_judge - before.total_judge,
            ));
        }
        ret
    }
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    play_count: PlayCount,
    clear_count: PlayCount,
    play_time: PlayTime,
    date: UpdatedAt,
    total_judge: TotalJudge,
}

impl PlayerState {
    pub fn new(
        play_count: PlayCount,
        clear_count: PlayCount,
        play_time: PlayTime,
        date: UpdatedAt,
        total_judge: TotalJudge,
    ) -> PlayerState {
        PlayerState {
            play_count,
            clear_count,
            play_time,
            date,
            total_judge,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerStateDiff {
    before_date: UpdatedAt,
    after_date: UpdatedAt,
    play_count: PlayCount,
    clear_count: PlayCount,
    play_time: PlayTime,
    total_judge: TotalJudge,
}

impl PlayerStateDiff {
    pub fn new(
        before_date: UpdatedAt,
        after_date: UpdatedAt,
        play_count: PlayCount,
        clear_count: PlayCount,
        play_time: PlayTime,
        total_judge: TotalJudge,
    ) -> PlayerStateDiff {
        PlayerStateDiff {
            before_date,
            after_date,
            play_count,
            clear_count,
            play_time,
            total_judge,
        }
    }
}

/// PlayTime(seconds)
#[derive(Debug, Clone, Serialize)]
pub struct PlayTime(i32);

impl PlayTime {
    pub fn new(seconds: i32) -> PlayTime {
        PlayTime(seconds)
    }
}

impl std::ops::Sub<PlayTime> for PlayTime {
    type Output = PlayTime;
    fn sub(self, rhs: PlayTime) -> PlayTime {
        PlayTime::new(self.0 - rhs.0)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TotalJudge(Judge);

impl TotalJudge {
    pub fn new(judge: Judge) -> TotalJudge {
        TotalJudge(judge)
    }
}

impl std::ops::Sub<TotalJudge> for TotalJudge {
    type Output = TotalJudge;
    fn sub(self, rhs: TotalJudge) -> TotalJudge {
        TotalJudge::new(self.0 - rhs.0)
    }
}
