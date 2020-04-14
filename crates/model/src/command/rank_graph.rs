use crate::*;
use std::borrow::Borrow;
use std::collections::HashMap;

pub(super) fn rank(
    songs: &Songs,
    table: &Table,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
    levels: &Levels,
) -> CommandResult {
    let mut vec = Vec::new();
    for level in &levels.levels {
        let specified = table.level_specified(level);
        let mut summary = Summary::new(ClearRank::vec());
        for song in specified.get_song(songs) {
            summary.push(
                &SongWithSnap::make(
                    &song,
                    score_log.get_snap(&song.song_id(), &updated_at).borrow(),
                )
                .clear_rank(),
            )
        }
        vec.push(RankCountByLevel {
            count: ClearRank::vec()
                .iter()
                .map(|c| {
                    (
                        c.clone(),
                        RankCountByType {
                            count: *summary.count(c).unwrap_or(&0i32),
                        },
                    )
                })
                .collect(),
        });
    }
    CommandResult::RankGraph(RankGraphResult {
        table: table.name(),
        vec: ClearRank::vec(),
        count: vec,
    })
}

#[derive(Deserialize, Serialize)]
pub struct RankGraphResult {
    table: String,
    vec: Vec<ClearRank>,
    count: Vec<RankCountByLevel>,
}

#[derive(Deserialize, Serialize)]
pub struct RankCountByLevel {
    count: HashMap<ClearRank, RankCountByType>,
}

#[derive(Deserialize, Serialize)]
pub struct RankCountByType {
    count: i32,
}

impl RankGraphResult {
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

impl RankCountByLevel {
    pub fn to_string(&self, vec: &Vec<ClearRank>) -> String {
        vec.iter()
            .flat_map(|c| match self.count.get(c) {
                Some(c) => Some(format!("[{:>3}]", c.to_string())),
                _ => None,
            })
            .collect::<String>()
    }
}

impl RankCountByType {
    pub fn to_string(&self) -> String {
        match self.count {
            0 => "".into(),
            _ => self.count.to_string(),
        }
    }
}
