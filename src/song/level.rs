use crate::command::Command;
use crate::score::scores::Scores;
use crate::score::updated_at::UpdatedAt;
use crate::score_log::ScoreLog;
use crate::song::Songs;
use crate::table::Table;
use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level {
    level: String,
}

impl Level {
    pub fn make(str: String) -> Level {
        Level {
            level: format!("{:>3}", str),
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.level.trim())
    }
}

pub struct Levels {
    levels: Vec<Level>,
}

impl Levels {
    pub fn new() -> Levels {
        Levels { levels: Vec::new() }
    }

    pub fn make(levels: Vec<Level>) -> Levels {
        Levels { levels }
    }

    pub fn format(
        &self,
        command: &Command,
        scores: &Scores,
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
    ) -> String {
        self.levels
            .iter()
            .map(|l| {
                command.func()(
                    scores,
                    songs,
                    &table.level_specified(l),
                    score_log,
                    &updated_at,
                )
            })
            .collect()
    }
}
