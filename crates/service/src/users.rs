use crate::Response;
use anyhow::Result;
use model::{ChangeNameQuery, ChangeVisibilityQuery, VisibleAccount};
use repository::{AccountByUserId, ChangeAccountVisibility, PublishedUsers, RenameAccount};
use schema::account::AccountResponse;
use session::Claims;

pub async fn my<R: AccountByUserId>(
    mut repos: R,
    claims: Claims,
) -> Result<Response<AccountResponse>> {
    Ok(Response::Ok {
        tag: None,
        body: AccountResponse::from_account(&repos.user(claims.user_id).await?),
    })
}

pub async fn change_name<C: RenameAccount + AccountByUserId>(
    mut repos: C,
    claims: Claims,
    query: ChangeNameQuery,
) -> Result<Response<AccountResponse>> {
    let mut account = repos.user(claims.user_id).await?;
    account.set_name(&query.changed_name);
    repos.rename(&account).await?;
    Ok(Response::Ok {
        tag: None,
        body: AccountResponse::from_account(&account),
    })
}

pub async fn change_visibility<C: ChangeAccountVisibility + AccountByUserId>(
    mut repos: C,
    claims: Claims,
    query: ChangeVisibilityQuery,
) -> Result<Response<AccountResponse>> {
    let mut account = repos.user(claims.user_id).await?;
    account.set_visibility(query.visibility);
    repos.change_visibility(&account).await?;
    Ok(Response::Ok {
        tag: None,
        body: AccountResponse::from_account(&account),
    })
}

pub async fn list<C: PublishedUsers>(mut repos: C) -> Result<Response<Vec<VisibleAccount>>> {
    Ok(Response::Ok {
        tag: None,
        body: repos.fetch_users().await?,
    })
}
