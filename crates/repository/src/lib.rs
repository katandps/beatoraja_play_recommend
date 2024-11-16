#![allow(async_fn_in_trait)]
use anyhow::Result;
use model::{
    Account, GoogleId, HashSha256, PlayerStats, RankedScore, Score, ScoreId, ScoreUpload, Scores,
    Songs, VisibleAccount,
};

pub trait PublishedUsers {
    async fn fetch_users(&mut self) -> Result<Vec<VisibleAccount>>;
}

pub trait HealthCheck {
    async fn health(&mut self) -> Result<()>;
}

pub trait AccountByUserId {
    async fn user(&mut self, user_id: i32) -> Result<Account>;
}

pub trait AccountByGoogleId {
    async fn user(&mut self, google_id: &GoogleId) -> Result<Account>;
}

pub trait ScoresByAccount {
    async fn score(&mut self, account: &Account) -> Result<Scores>;
}

pub trait ScoresBySha256 {
    async fn score(&mut self, hash: &HashSha256) -> Result<RankedScore>;
}

pub trait ScoreByAccountAndSha256 {
    async fn score_with_log(&mut self, account: &Account, score_id: &ScoreId) -> Result<Score>;
}

pub trait StatsByAccount {
    async fn stats(&mut self, account: &Account) -> Result<PlayerStats>;
}

pub trait RenameAccount {
    async fn rename(&mut self, account: &Account) -> Result<()>;
}

pub trait ChangeAccountVisibility {
    async fn change_visibility(&mut self, account: &Account) -> Result<()>;
}

pub trait AllSongData {
    async fn song_data(&mut self) -> Result<Songs>;
}

pub trait RegisterUpload {
    async fn register(&mut self) -> Result<ScoreUpload>;
}

pub trait SaveSongData {
    async fn save_song(&mut self, songs: &Songs) -> Result<()>;
}

pub trait SaveScoreData {
    async fn save_score(&mut self, account: &Account, score: &Scores) -> Result<()>;
}

pub trait SavePlayerStateData {
    async fn save_player_states(&mut self, account: &Account, states: &PlayerStats) -> Result<()>;
}

pub trait ResetScore {
    async fn reset_score(&mut self, account: &Account) -> Result<()>;
}
