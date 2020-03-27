pub struct WholeScore {
    scores: Scores
}

impl WholeScore {
    pub fn new(scores: Scores) -> WholeScore { WholeScore { scores } }

    pub fn count(&self) -> usize { self.scores.count() }
}

pub struct Scores {
    scores: Vec<Score>
}

impl Scores {
    pub fn new(scores: Vec<Score>) -> Scores { Scores { scores } }
    pub fn count(&self) -> usize { self.scores.len() }
}

pub struct Score {
    id: SongId,
    clear: ClearType,
}

impl Score {
    pub fn new(id: SongId, clear: ClearType) -> Score { Score { id, clear } }
}

pub struct SongId {
    sha256: String,
    mode: i32,
}

impl SongId {
    pub fn new(sha256: String, mode: i32) -> SongId {
        SongId { sha256, mode }
    }
}

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