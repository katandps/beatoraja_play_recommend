use crate::command::Command;
use crate::score::updated_at::UpdatedAt;
use crate::score_log::ScoreLog;
use crate::song::Songs;
use crate::table::Table;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
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
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
    ) -> String {
        self.levels
            .iter()
            .map(|level| {
                command.func()(
                    songs,
                    &table.level_specified(level),
                    score_log,
                    &updated_at,
                    level,
                )
                .to_string()
            })
            .collect()
    }
}
