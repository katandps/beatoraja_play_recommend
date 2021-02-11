use crate::error::HandleError;
use mysql::PublishedUsers;
use mysql::{MySQLClient, VisibleAccount};
use warp::{Rejection, Reply};

pub async fn users_handler(repos: MySQLClient) -> std::result::Result<impl Reply, Rejection> {
    let users = fetch_users(repos)?;
    Ok(serde_json::to_string(&users).unwrap())
}

fn fetch_users(repos: MySQLClient) -> Result<Vec<VisibleAccount>, HandleError> {
    let users = repos.fetch_users()?;
    Ok(users)
}
