use crate::error::HandleError;
use crate::error::HandleError::ChangedNameNotFound;
use model::Account;
use mysql::MySQLClient;
use std::collections::HashMap;
use warp::{Rejection, Reply};

pub async fn change_name_handler(
    repos: MySQLClient,
    session_key: String,
    request_body: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let account = crate::session::get_account_by_session(&repos, &session_key)?;
    let changed_name = request_body
        .get(&"changed_name".to_string())
        .ok_or(ChangedNameNotFound)?;
    let mut new = account.clone();
    new.set_name(changed_name.clone());
    rename_account(&repos, &new)?;
    Ok(serde_json::to_string(&new).unwrap())
}

fn rename_account(repos: &MySQLClient, account: &Account) -> Result<(), HandleError> {
    Ok(repos.rename_account(account)?)
}
