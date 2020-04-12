use super::*;
use serde::{Deserialize, Serialize};

pub(super) fn lamp(
    songs: &Songs,
    table: &Table,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
    levels: &Levels,
) -> CommandResult {
    let mut str = String::new();
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
        str.push_str(format!("{}", summary).as_str());
    }

    CommandResult::LampGraph(LampGraphResult {
        table: table.name(),
        count: Vec::new(),
    })
}

#[derive(Deserialize, Serialize)]
pub struct LampGraphResult {
    table: String,
    count: Vec<LampCountByLamp>,
}

#[derive(Deserialize, Serialize)]
pub struct LampCountByLamp {
    lamp_type: ClearType,
    count: Vec<LampCountByLevel>,
}

#[derive(Deserialize, Serialize)]
pub struct LampCountByLevel {
    count: i32,
}

impl LampGraphResult {
    pub fn to_string(&self) -> String {
        self.table.clone()
    }
}
