use crate::Response;
use anyhow::Result;
use model::{PlayerStats, ScoreUpload};
use repository::{AccountByUserId, StatsByDays, UploadsByDays};
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

pub async fn by_user_uploads<C: AccountByUserId + UploadsByDays>(
    mut repos: C,
    claims: Claims,
) -> Result<Response<Vec<ScoreUpload>>> {
    let account = repos.user(claims.user_id).await?;
    let uploads = repos.uploads(&account).await?;
    Ok(Response::Ok {
        tag: None,
        body: uploads,
    })
}
