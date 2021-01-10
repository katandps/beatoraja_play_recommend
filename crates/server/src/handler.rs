pub mod change_name;
pub mod detail;
pub mod health;
pub mod oauth_redirect;
pub mod upload;

use http::StatusCode;
use model::*;
use mysql::MySQLClient;
use std::collections::HashMap;
use warp::{Rejection, Reply};

pub async fn table_handler(tables: Tables) -> std::result::Result<impl Reply, Rejection> {
    Ok(serde_json::to_string(&tables.format()).unwrap())
}

pub async fn account_handler(
    repos: MySQLClient,
    session_key: String,
) -> Result<impl Reply, Rejection> {
    let account = crate::session::get_account_by_session(&repos, &session_key)?;
    Ok(serde_json::to_string(&account).unwrap())
}

pub async fn logout_handler(session_key: String) -> Result<impl Reply, Rejection> {
    crate::session::remove_session(&session_key)?;
    Ok(StatusCode::OK)
}

fn date(map: &HashMap<String, String>) -> UpdatedAt {
    if let Some(date) = map.get("date".into()) {
        UpdatedAt::from_str(date).sub(-1)
    } else {
        UpdatedAt::new()
    }
}
