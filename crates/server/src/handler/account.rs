use model::Account;
use warp::{Rejection, Reply};

pub async fn account_handler(account: Account) -> Result<impl Reply, Rejection> {
    Ok(serde_json::to_string(&account).unwrap())
}
