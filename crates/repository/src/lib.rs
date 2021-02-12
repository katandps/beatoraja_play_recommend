use anyhow::Result;
use model::{Account, Scores};
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
    fn user(&self, google_id: i32) -> Result<Account>;
}

pub trait ScoresByAccount {
    fn score(&self, account: &Account) -> Result<Scores>;
}
