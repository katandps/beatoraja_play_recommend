use crate::{Judge, PlayCount, UpdatedAt, UserId};
use serde::Serialize;
use std::cmp::Ordering;

#[derive(Debug, Serialize)]
pub struct PlayerStats {
    pub log: Vec<PlayerStat>,
}

impl PlayerStats {
    pub fn new(log: Vec<PlayerStat>) -> PlayerStats {
        PlayerStats { log }
    }

    pub fn last(&self) -> Option<&PlayerStat> {
        self.log.iter().last()
    }

    pub fn diff(&self) -> Vec<PlayerStatDiff> {
        let mut log = self.log.clone();
        log.sort_by(PlayerStat::cmp_by_date);
        let mut ret = Vec::new();
        for i in 1..log.len() {
            let before = log[i - 1].clone();
            let after = log[i].clone();
            ret.push(PlayerStatDiff::new(
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

#[derive(Debug, Clone, Serialize)]
pub struct PlayerStat {
    pub play_count: PlayCount,
    pub clear_count: PlayCount,
    pub play_time: PlayTime,
    pub date: UpdatedAt,
    pub total_judge: TotalJudge,
}

impl PlayerStat {
    pub fn new(
        play_count: PlayCount,
        clear_count: PlayCount,
        play_time: PlayTime,
        date: UpdatedAt,
        total_judge: TotalJudge,
    ) -> PlayerStat {
        PlayerStat {
            play_count,
            clear_count,
            play_time,
            date,
            total_judge,
        }
    }

    pub fn cmp_by_date(&self, other: &PlayerStat) -> Ordering {
        self.date.cmp(&other.date)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerStatDiff {
    before_date: UpdatedAt,
    after_date: UpdatedAt,
    play_count: PlayCount,
    clear_count: PlayCount,
    play_time: PlayTime,
    total_judge: TotalJudge,
}

impl PlayerStatDiff {
    pub fn new(
        before_date: UpdatedAt,
        after_date: UpdatedAt,
        play_count: PlayCount,
        clear_count: PlayCount,
        play_time: PlayTime,
        total_judge: TotalJudge,
    ) -> PlayerStatDiff {
        PlayerStatDiff {
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
pub struct PlayTime(pub i32);

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

    pub fn judge(&self) -> &Judge {
        &self.0
    }
}

impl std::ops::Sub<TotalJudge> for TotalJudge {
    type Output = TotalJudge;
    fn sub(self, rhs: TotalJudge) -> TotalJudge {
        TotalJudge::new(self.0 - rhs.0)
    }
}

#[derive(Serialize)]
pub struct VisibleAccount {
    pub id: UserId,
    pub name: String,
}
