use anyhow::Result;
use model::{Account, GoogleId, Scores, Songs};
use serde::Serialize;

pub trait PublishedUsers {
    fn fetch_users(&self) -> Result<Vec<VisibleAccount>>;
}

#[derive(Serialize)]
pub struct VisibleAccount {
    pub id: i32,
    pub name: String,
}

pub trait HealthCheck {
    fn health(&self) -> Result<()>;
}

pub trait AccountByIncrement {
    fn user(&self, increment: i32) -> Result<Account>;
}

pub trait AccountByGoogleId {
    fn user(&self, google_id: &GoogleId) -> Result<Account>;
}

pub trait ScoresByAccount {
    fn score(&self, account: &Account) -> Result<Scores>;
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
