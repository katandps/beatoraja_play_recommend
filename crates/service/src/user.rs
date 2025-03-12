use anyhow::Result;
use model::Account;
use repository::RenameAccount;

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
