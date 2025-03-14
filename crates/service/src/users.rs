use anyhow::Result;
use model::{Account, ChangeNameQuery, ChangeVisibilityQuery, VisibleAccount};
use repository::{AccountByUserId, ChangeAccountVisibility, PublishedUsers, RenameAccount};
use session::Claims;

use crate::Response;

pub async fn my<R: AccountByUserId>(mut repos: R, claims: Claims) -> Result<Response<Account>> {
    Ok(Response::Ok {
        tag: None,
        body: repos.user(claims.user_id).await?,
    })
}

pub async fn change_name<C: RenameAccount + AccountByUserId>(
    mut repos: C,
    claims: Claims,
    query: ChangeNameQuery,
) -> Result<Response<Account>> {
    let mut account = repos.user(claims.user_id).await?;
    account.set_name(&query.changed_name);
    let _ = repos.rename(&account).await?;
    Ok(Response::Ok {
        tag: None,
        body: account,
    })
}

pub async fn change_visibility<C: ChangeAccountVisibility + AccountByUserId>(
    mut repos: C,
    claims: Claims,
    query: ChangeVisibilityQuery,
) -> Result<Response<Account>> {
    let mut account = repos.user(claims.user_id).await?;
    account.set_visibility(query.visibility);
    repos.change_visibility(&account).await?;
    Ok(Response::Ok {
        tag: None,
        body: account,
    })
}

pub async fn list<C: PublishedUsers>(mut repos: C) -> Result<Response<Vec<VisibleAccount>>> {
    Ok(Response::Ok {
        tag: None,
        body: repos.fetch_users().await?,
    })
}
