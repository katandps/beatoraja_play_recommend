use crate::error::HandleError;
use model::Account;
use mysql::MySQLClient;
use warp::{Rejection, Reply};

pub async fn change_visibility_handler(
    repos: MySQLClient,
    mut account: Account,
    changed_visibility: bool,
) -> Result<impl Reply, Rejection> {
    account.set_visibility(changed_visibility);
    change_visibility(&repos, &account)?;
    Ok(serde_json::to_string(&account).unwrap())
}

fn change_visibility(repos: &MySQLClient, account: &Account) -> Result<(), HandleError> {
    Ok(repos.change_account_visibility(account)?)
}
