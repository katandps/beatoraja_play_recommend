use crate::error::HandleError;
use model::Account;
use repository::RenameAccount;
use warp::{Rejection, Reply};

pub async fn change_name_handler<C: RenameAccount>(
    repos: C,
    mut account: Account,
    changed_name: String,
) -> Result<impl Reply, Rejection> {
    account.set_name(&changed_name);
    rename_account(&repos, &account)?;
    Ok(serde_json::to_string(&account).unwrap())
}

fn rename_account<C: RenameAccount>(repos: &C, account: &Account) -> Result<(), HandleError> {
    Ok(repos.rename(account)?)
}
