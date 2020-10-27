use crate::{Judge, PlayCount, UpdatedAt};

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
}

#[derive(Debug)]
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

/// PlayTime(seconds)
#[derive(Debug)]
pub struct PlayTime(i32);

impl PlayTime {
    pub fn new(seconds: i32) -> PlayTime {
        PlayTime(seconds)
    }
}

#[derive(Debug)]
pub struct TotalJudge {
    judge: Judge,
}

impl TotalJudge {
    pub fn new(judge: Judge) -> TotalJudge {
        TotalJudge { judge }
    }
}
