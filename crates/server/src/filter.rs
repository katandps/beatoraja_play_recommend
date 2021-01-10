use crate::error::HandleError;
use model::*;
use mysql::{MySQLClient, MySqlPool};
use oauth_google::GoogleProfile;
use std::collections::HashMap;
use std::convert::Infallible;
use warp::filters::multipart::FormData;
use warp::{Filter, Rejection};

pub fn with_db(
    db_pool: MySqlPool,
) -> impl Filter<Extract = (MySQLClient,), Error = Infallible> + Clone {
    warp::any().map(move || MySQLClient::new(db_pool.clone().get().unwrap()))
}

pub fn with_table(tables: Tables) -> impl Filter<Extract = (Tables,), Error = Infallible> + Clone {
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
