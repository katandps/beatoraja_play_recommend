use crate::error::HandleError;
use model::Account;
use mysql::MySQLClient;
use warp::{Rejection, Reply};

pub async fn change_name_handler(
    repos: MySQLClient,
    mut account: Account,
    changed_name: String,
) -> Result<impl Reply, Rejection> {
    account.set_name(&changed_name);
    rename_account(&repos, &account)?;
    Ok(serde_json::to_string(&account).unwrap())
}

fn rename_account(repos: &MySQLClient, account: &Account) -> Result<(), HandleError> {
    Ok(repos.rename_account(account)?)
}
