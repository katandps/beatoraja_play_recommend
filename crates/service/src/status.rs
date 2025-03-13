use crate::Response;
use anyhow::Result;
use model::{PlayerStats, UserId};
use repository::{AccountByUserId, StatsByAccount};

pub async fn by_user<C: AccountByUserId + StatsByAccount>(
    user_id: i32,
    mut repos: C,
) -> Result<Response<PlayerStats>> {
    let account = repos.user(UserId::new(user_id)).await?;
    let stats = repos.stats(&account).await?;
    Ok(Response::Ok {
        tag: None,
        body: stats,
    })
}
