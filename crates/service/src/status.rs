use crate::Response;
use anyhow::Result;
use model::PlayerStats;
use repository::{AccountByUserId, StatsByDays};
use session::Claims;

pub async fn by_user<C: AccountByUserId + StatsByDays>(
    mut repos: C,
    claims: Claims,
) -> Result<Response<PlayerStats>> {
    let account = repos.user(claims.user_id).await?;
    let stats = repos.stats(&account).await?;
    Ok(Response::Ok {
        tag: None,
        body: stats,
    })
}
