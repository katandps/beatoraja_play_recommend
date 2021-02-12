use crate::error::HandleError;
use repository::{PublishedUsers, VisibleAccount};
use warp::{Rejection, Reply};

pub async fn users_handler<C: PublishedUsers>(
    repos: C,
) -> std::result::Result<impl Reply, Rejection> {
    let users = fetch_users(repos)?;
    Ok(serde_json::to_string(&users).unwrap())
}

fn fetch_users<C: PublishedUsers>(repos: C) -> Result<Vec<VisibleAccount>, HandleError> {
    let users = repos.fetch_users()?;
    Ok(users)
}
