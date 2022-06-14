use anyhow::Result;
use model::{
    Account, GoogleId, HashSha256, PlayerStats, RankedScore, Score, ScoreId, Scores, Songs,
    VisibleAccount,
};

pub trait PublishedUsers {
    fn fetch_users(&self) -> Result<Vec<VisibleAccount>>;
}

pub trait HealthCheck {
    fn health(&self) -> Result<()>;
}

pub trait AccountByUserId {
    fn user(&self, user_id: i32) -> Result<Account>;
}

pub trait AccountByGoogleId {
    fn user(&self, google_id: &GoogleId) -> Result<Account>;
}

pub trait ScoresByAccount {
    fn score(&self, account: &Account) -> Result<Scores>;
}

pub trait ScoresBySha256 {
    fn score(&self, hash: &HashSha256) -> Result<RankedScore>;
}

pub trait ScoreByAccountAndSha256 {
    fn score_with_log(&self, account: &Account, score_id: &ScoreId) -> Result<Score>;
}

pub trait StatsByAccount {
    fn stats(&self, account: &Account) -> Result<PlayerStats>;
}

pub trait RenameAccount {
    fn rename(&self, account: &Account) -> Result<()>;
}

pub trait ChangeAccountVisibility {
    fn change_visibility(&self, account: &Account) -> Result<()>;
}

pub trait AllSongData {
    fn song_data(&self) -> Result<Songs>;
}

pub trait SaveSongData {
    fn save_song(&self, songs: &Songs) -> Result<()>;
}

pub trait SaveScoreData {
    fn save_score(&self, account: &Account, score: &Scores) -> Result<()>;
}

pub trait SavePlayerStateData {
    fn save_player_states(&self, account: &Account, states: &PlayerStats) -> Result<()>;
}

pub trait ResetScore {
    fn reset_score(&self, account: &Account) -> Result<()>;
}
