use crate::summary::Countable;
use crate::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ClearType {
    NoPlay,
    Failed,
    AssistEasy,
    LightAssistEasy,
    Easy,
    Normal,
    Hard,
    ExHard,
    FullCombo,
    Perfect,
    Max,
    Unknown,
}

impl ClearType {
    pub fn from_integer(int: i32) -> ClearType {
        match int {
            0 => ClearType::NoPlay,
            1 => ClearType::Failed,
            2 => ClearType::AssistEasy,
            3 => ClearType::LightAssistEasy,
            4 => ClearType::Easy,
            5 => ClearType::Normal,
            6 => ClearType::Hard,
            7 => ClearType::ExHard,
            8 => ClearType::FullCombo,
            9 => ClearType::Perfect,
            10 => ClearType::Max,
            _ => ClearType::Unknown,
        }
    }
}

impl Countable for ClearType {
    fn coloring(&self, s: String) -> String {
        const ESC: &str = "\u{001b}";
        match self {
            ClearType::NoPlay => format!("{}", s),
            ClearType::Failed => format!("{ESC}[00;31m{}{ESC}[00m", s, ESC = ESC),
            ClearType::AssistEasy => format!("{ESC}[00;34m{}{ESC}[00m", s, ESC = ESC),
            ClearType::LightAssistEasy => format!("{ESC}[00;35m{}{ESC}[00m", s, ESC = ESC),
            ClearType::Easy => format!("{ESC}[00;32m{}{ESC}[00m", s, ESC = ESC),
            ClearType::Normal => format!("{ESC}[00;36m{}{ESC}[00m", s, ESC = ESC),
            ClearType::Hard => format!("{ESC}[00;40m{}{ESC}[00m", s, ESC = ESC),
            ClearType::ExHard => format!("{ESC}[00;33m{}{ESC}[00m", s, ESC = ESC),
            ClearType::FullCombo => format!("{ESC}[00;1;46m{}{ESC}[00m", s, ESC = ESC),
            ClearType::Perfect => format!("{ESC}[00;1;43m{}{ESC}[00m", s, ESC = ESC),
            ClearType::Max => format!("{ESC}[00;1m{}{ESC}[00m", s, ESC = ESC),
            ClearType::Unknown => format!("{}", s),
        }
    }
    fn vec() -> Vec<ClearType> {
        (0..11).map(|x| ClearType::from_integer(x)).collect()
    }
    fn get_from(song: &Song, score_log: &ScoreLog, updated_at: &UpdatedAt) -> Self {
        score_log
            .get_snap(&song.song_id(), &updated_at)
            .clear_type()
            .clone()
    }
}

impl fmt::Display for ClearType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ClearType::NoPlay => write!(f, "NoPlay"),
            ClearType::Failed => write!(f, "Failed"),
            ClearType::AssistEasy => write!(f, "AssistEasy"),
            ClearType::LightAssistEasy => write!(f, "LightAssistEasy"),
            ClearType::Easy => write!(f, "Easy"),
            ClearType::Normal => write!(f, "Normal"),
            ClearType::Hard => write!(f, "Hard"),
            ClearType::ExHard => write!(f, "ExHard"),
            ClearType::FullCombo => write!(f, "FullCombo"),
            ClearType::Perfect => write!(f, "Perfect"),
            ClearType::Max => write!(f, "Max"),
            ClearType::Unknown => write!(f, "Unknown"),
        }
    }
}
