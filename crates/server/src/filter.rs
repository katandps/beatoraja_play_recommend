use crate::error::HandleError;
use model::*;
use mysql::{MySQLClient, MySqlPool};
use oauth_google::GoogleProfile;
use std::collections::HashMap;
use std::convert::Infallible;
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

pub fn receive_sqlite_file() -> impl Filter<Extract = (FormData,), Error = Rejection> + Clone {
    warp::multipart::form().max_length(100 * 1024 * 1024)
}

pub fn receive_session_key() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::header::<String>(crate::session::SESSION_KEY)
}

pub fn google_oauth_code() -> impl Filter<Extract = (GoogleProfile,), Error = Rejection> + Clone {
    warp::query::<HashMap<String, String>>().and_then(verify)
}

async fn verify(query: HashMap<String, String>) -> Result<GoogleProfile, Rejection> {
    let code = query
        .get(&"code".to_string())
        .cloned()
        .ok_or(HandleError::AuthorizationCodeIsNotFound)?;
    let profile = oauth_google::verify(code)
        .await
        .map_err(|e| HandleError::OAuthGoogleError(e))?;
    Ok(profile)
}

pub fn detail_query() -> impl Filter<Extract = (DetailQuery,), Error = Rejection> + Clone {
    warp::query::<HashMap<String, String>>().and_then(parse_detail_query)
}

async fn parse_detail_query(query: HashMap<String, String>) -> Result<DetailQuery, Rejection> {
    let date = if let Some(date) = query.get("date".into()) {
        UpdatedAt::from_str(date).sub(-1)
    } else {
        UpdatedAt::new()
    };
    let play_mode = if let Some(mode) = query.get("mode".into()) {
        match mode.parse::<i32>() {
            Ok(mode) => PlayMode::new(mode),
            Err(_) => PlayMode::default(),
        }
    } else {
        PlayMode::default()
    };
    Ok(DetailQuery { date, play_mode })
}

pub fn account_id_query(
    db_pool: &MySqlPool,
) -> impl Filter<Extract = (Account,), Error = Rejection> + Clone {
    with_db(&db_pool)
        .and(warp::query::<HashMap<String, String>>())
        .and_then(get_account_by_query)
}

async fn get_account_by_query(
    repos: MySQLClient,
    query: HashMap<String, String>,
) -> Result<Account, Rejection> {
    let user_id = query
        .get(&"user_id".to_string())
        .ok_or(HandleError::AccountIsNotSelected)?;
    let user_id = user_id
        .parse::<i32>()
        .map_err(|e| HandleError::AccountSelectionIsInvalid(e))?;
    let account = repos
        .account_by_increments(user_id)
        .map_err(|e| HandleError::AccountIsNotFound(e))?;
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

async fn get_changed_name_query(body: HashMap<String, String>) -> Result<String, Rejection> {
    let changed_name = body
        .get(&"changed_name".to_string())
        .ok_or(HandleError::ChangedNameNotFound)?;
    Ok(changed_name.clone())
}

pub struct DetailQuery {
    pub date: UpdatedAt,
    pub play_mode: PlayMode,
}
