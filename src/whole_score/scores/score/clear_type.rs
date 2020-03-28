use std::fmt;

#[derive(Clone)]
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
            _ => ClearType::Unknown
        }
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