use anyhow::Result;
use model::{Account, VisibleAccount};
use repository::{ChangeAccountVisibility, PublishedUsers, RenameAccount};

use crate::Response;

pub async fn change_name<C: RenameAccount>(
    mut repos: C,
    mut account: Account,
    changed_name: String,
) -> Result<Response<Account>> {
    account.set_name(&changed_name);
    let _ = repos.rename(&account).await?;
    Ok(Response::Ok {
        tag: None,
        body: account,
    })
}

pub async fn change_visibility<C: ChangeAccountVisibility>(
    mut repos: C,
    mut account: Account,
    changed_visibility: bool,
) -> Result<Response<Account>> {
    account.set_visibility(changed_visibility);
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
