use crate::error::HandleError;
use crate::TableData;
use model::*;
use mysql::{MySQLClient, MySqlPool};
use repository::AccountByUserId;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use warp::filters::multipart::FormData;
use warp::{Filter, Rejection};

pub fn with_db(
    db_pool: &MySqlPool,
) -> impl Filter<Extract = (MySQLClient,), Error = Infallible> + Clone {
    let db_pool = db_pool.clone();
    warp::any().map(move || MySQLClient::new(db_pool.get().unwrap()))
}

pub fn with_table(
    tables: &TableData,
) -> impl Filter<Extract = (TableData,), Error = Infallible> + Clone {
    let tables = Arc::clone(tables);
    warp::any().map(move || tables.clone())
}

pub fn with_tag() -> impl Filter<Extract = (Option<String>,), Error = Rejection> + Clone {
    warp::header::optional::<String>("If-None-Match")
}

pub fn receive_sqlite_file() -> impl Filter<Extract = (FormData,), Error = Rejection> + Clone {
    warp::multipart::form().max_length(100 * 1024 * 1024)
}

pub fn receive_session_key() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::header::<String>(crate::session::SESSION_KEY)
}

pub fn account_id_query(
    db_pool: &MySqlPool,
) -> impl Filter<Extract = (Account,), Error = Rejection> + Clone {
    with_db(db_pool)
        .and(warp::query::<HashMap<String, String>>())
        .and_then(get_account_by_query)
}

async fn get_account_by_query<C: AccountByUserId>(
    mut repos: C,
    query: HashMap<String, String>,
) -> Result<Account, Rejection> {
    let user_id = query
        .get("user_id")
        .ok_or(HandleError::AccountIsNotSelected)?;
    let user_id = user_id
        .parse::<i32>()
        .map_err(HandleError::AccountSelectionIsInvalid)?;
    let account = repos
        .user(user_id)
        .await
        .map_err(HandleError::AccountIsNotFound)?;
    Ok(account)
}

pub fn account_by_session(
    db_pool: &MySqlPool,
) -> impl Filter<Extract = (Account,), Error = Rejection> + Clone {
    with_db(db_pool)
        .and(receive_session_key())
        .and_then(crate::session::get_account_by_session)
}

pub fn changed_name_by_query() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::body::json().and_then(get_changed_name_query)
}

pub fn changed_visibility_by_query() -> impl Filter<Extract = (bool,), Error = Rejection> + Clone {
    warp::body::json().and_then(get_changed_visibility_query)
}

async fn get_changed_name_query(body: HashMap<String, String>) -> Result<String, Rejection> {
    let changed_name = body
        .get("changed_name")
        .ok_or(HandleError::ChangedNameNotFound)?;
    Ok(changed_name.clone())
}

async fn get_changed_visibility_query(body: HashMap<String, String>) -> Result<bool, Rejection> {
    let changed_visibility = body
        .get("visibility")
        .ok_or(HandleError::ChangedVisibilityNotFound)?;
    Ok(changed_visibility == &"true".to_string())
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DetailQuery {
    pub date: UpdatedAt,
    #[serde(default)]
    pub play_mode: PlayMode,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct RankingQuery {
    pub date: UpdatedAt,
    #[serde(default)]
    pub play_mode: PlayMode,
    pub sha256: HashSha256,
}

#[derive(Deserialize)]
pub struct SongLogQuery {
    #[serde(default)]
    pub play_mode: PlayMode,
    pub sha256: HashSha256,
}
