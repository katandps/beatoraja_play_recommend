use anyhow::Result;
use model::Account;
use repository::{ChangeAccountVisibility, RenameAccount};

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
