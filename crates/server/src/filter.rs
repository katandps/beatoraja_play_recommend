use crate::error::HandleError;
use crate::SongData;
use model::*;
use mysql::{MySQLClient, MySqlPool};
use repository::AccountByUserId;
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

pub fn with_table(tables: &Tables) -> impl Filter<Extract = (Tables,), Error = Infallible> + Clone {
    let tables = tables.clone();
    warp::any().map(move || tables.clone())
}

pub fn with_song_data(
    song_data: &SongData,
) -> impl Filter<Extract = (SongData,), Error = Infallible> + Clone {
    let song_data = Arc::clone(song_data);
    warp::any().map(move || song_data.clone())
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
    repos: C,
    query: HashMap<String, String>,
) -> Result<Account, Rejection> {
    let user_id = query
        .get(&"user_id".to_string())
        .ok_or(HandleError::AccountIsNotSelected)?;
    let user_id = user_id
        .parse::<i32>()
        .map_err(HandleError::AccountSelectionIsInvalid)?;
    let account = repos
        .user(user_id)
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
        .get(&"changed_name".to_string())
        .ok_or(HandleError::ChangedNameNotFound)?;
    Ok(changed_name.clone())
}

async fn get_changed_visibility_query(body: HashMap<String, String>) -> Result<bool, Rejection> {
    let changed_visibility = body
        .get(&"visibility".to_string())
        .ok_or(HandleError::ChangedVisibilityNotFound)?;
    Ok(changed_visibility == &"true".to_string())
}

pub struct DetailQuery {
    pub date: UpdatedAt,
    pub play_mode: PlayMode,
}

pub struct RankingQuery {
    pub date: UpdatedAt,
    pub play_mode: PlayMode,
    pub sha256: HashSha256,
}
