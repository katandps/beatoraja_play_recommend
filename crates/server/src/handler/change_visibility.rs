use crate::error::HandleError;
use model::Account;
use repository::ChangeAccountVisibility;
use warp::{Rejection, Reply};

pub async fn change_visibility_handler<C: ChangeAccountVisibility>(
    repos: C,
    mut account: Account,
    changed_visibility: bool,
) -> Result<impl Reply, Rejection> {
    account.set_visibility(changed_visibility);
    change_visibility(&repos, &account)?;
    Ok(serde_json::to_string(&account).unwrap())
}

fn change_visibility<C: ChangeAccountVisibility>(
    repos: &C,
    account: &Account,
) -> Result<(), HandleError> {
    Ok(repos.change_visibility(account)?)
}
