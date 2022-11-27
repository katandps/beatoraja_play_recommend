use anyhow::Result;
use model::{
    Account, GoogleId, HashSha256, PlayerStats, RankedScore, Score, ScoreId, Scores, Songs,
    VisibleAccount,
};

pub trait PublishedUsers {
    fn fetch_users(&mut self) -> Result<Vec<VisibleAccount>>;
}

pub trait HealthCheck {
    fn health(&mut self) -> Result<()>;
}

pub trait AccountByUserId {
    fn user(&mut self, user_id: i32) -> Result<Account>;
}

pub trait AccountByGoogleId {
    fn user(&mut self, google_id: &GoogleId) -> Result<Account>;
}

pub trait ScoresByAccount {
    fn score(&mut self, account: &Account) -> Result<Scores>;
}

pub trait ScoresBySha256 {
    fn score(&mut self, hash: &HashSha256) -> Result<RankedScore>;
}

pub trait ScoreByAccountAndSha256 {
    fn score_with_log(&mut self, account: &Account, score_id: &ScoreId) -> Result<Score>;
}

pub trait StatsByAccount {
    fn stats(&mut self, account: &Account) -> Result<PlayerStats>;
}

pub trait RenameAccount {
    fn rename(&mut self, account: &Account) -> Result<()>;
}

pub trait ChangeAccountVisibility {
    fn change_visibility(&mut self, account: &Account) -> Result<()>;
}

pub trait AllSongData {
    fn song_data(&mut self) -> Result<Songs>;
}

pub trait SaveSongData {
    fn save_song(&mut self, songs: &Songs) -> Result<()>;
}

pub trait SaveScoreData {
    fn save_score(&mut self, account: &Account, score: &Scores) -> Result<()>;
}

pub trait SavePlayerStateData {
    fn save_player_states(&mut self, account: &Account, states: &PlayerStats) -> Result<()>;
}

pub trait ResetScore {
    fn reset_score(&mut self, account: &Account) -> Result<()>;
}
