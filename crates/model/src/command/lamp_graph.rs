use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(super) fn lamp(
    songs: &Songs,
    table: &Table,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
    levels: &Levels,
) -> CommandResult {
    let mut vec = Vec::new();
    for level in &levels.levels {
        let specified = table.level_specified(level);
        let mut summary = Summary::new(ClearType::vec());
        for song in specified.get_song(songs) {
            summary.push(
                score_log
                    .get_snap(&song.song_id(), &updated_at)
                    .clear_type(),
            )
        }
        vec.push(LampCountByLevel {
            count: ClearType::vec()
                .iter()
                .map(|c| {
                    (
                        c.clone(),
                        LampCountByType {
                            count: *summary.count(c).unwrap_or(&0i32),
                        },
                    )
                })
                .collect(),
        });
    }

    CommandResult::LampGraph(LampGraphResult {
        table: table.name(),
        vec: ClearType::vec(),
        count: vec,
    })
}

#[derive(Deserialize, Serialize)]
pub struct LampGraphResult {
    table: String,
    vec: Vec<ClearType>,
    count: Vec<LampCountByLevel>,
}

#[derive(Deserialize, Serialize)]
pub struct LampCountByLevel {
    count: HashMap<ClearType, LampCountByType>,
}

#[derive(Deserialize, Serialize)]
pub struct LampCountByType {
    count: i32,
}

impl LampGraphResult {
    pub fn to_string(&self) -> String {
        self.table.clone()
            + "\n"
            + self
                .count
                .iter()
                .map(|l| l.to_string(&self.vec) + "\n")
                .collect::<String>()
                .as_str()
    }
}

impl LampCountByLevel {
    pub fn to_string(&self, vec: &Vec<ClearType>) -> String {
        vec.iter()
            .flat_map(|c| match self.count.get(c) {
                Some(c) => Some(format!("[{:>3}]", c.to_string())),
                _ => None,
            })
            .collect::<String>()
    }
}

impl LampCountByType {
    pub fn to_string(&self) -> String {
        match self.count {
            0 => "".into(),
            _ => self.count.to_string(),
        }
    }
}
