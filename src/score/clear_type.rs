use std::fmt;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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

    pub fn vec() -> Vec<ClearType> {
        (0..11).map(|x| ClearType::from_integer(x)).collect()
    }

    pub fn coloring(&self, s: String) -> String {
        const ESC: &str = "\u{001b}";
        match self {
            ClearType::NoPlay => format!("{}", s),
            ClearType::Failed => format!("{ESC}[00;31m{}{ESC}[00m", s, ESC = ESC),
            ClearType::AssistEasy => format!("{ESC}[00;34m{}{ESC}[00m", s, ESC = ESC),
            ClearType::LightAssistEasy => format!("\u{001b}[00;35m{}\u{001b}[00m", s),
            ClearType::Easy => format!("\u{001b}[00;32m{}\u{001b}[00m", s),
            ClearType::Normal => format!("\u{001b}[00;36m{}\u{001b}[00m", s),
            ClearType::Hard => format!("\u{001b}[00;40m{}\u{001b}[00m", s),
            ClearType::ExHard => format!("\u{001b}[00;33m{}\u{001b}[00m", s),
            ClearType::FullCombo => format!("\u{001b}[00;1;46m{}\u{001b}[00m", s),
            ClearType::Perfect => format!("\u{001b}[00;1;43m{}\u{001b}[00m", s),
            ClearType::Max => format!("\u{001b}[00;1m{}\u{001b}[00m", s),
            ClearType::Unknown => format!("{}", s),
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
