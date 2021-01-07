pub mod detail;
pub mod health;
pub mod upload;

use crate::config::config;
use crate::error::HandleError::{ChangedNameNotFound, OtherError};
use crate::error::*;
use crate::session::save_user_id;
use http::StatusCode;
use model::*;
use mysql::MySQLClient;
use std::collections::HashMap;
use warp::http::Uri;
use warp::{Rejection, Reply};

pub async fn table_handler(tables: Tables) -> std::result::Result<impl Reply, Rejection> {
    Ok(serde_json::to_string(&tables.format()).unwrap())
}

pub async fn account_handler(session_key: String) -> Result<impl Reply, Rejection> {
    match crate::session::get_account_by_session(&session_key) {
        Ok(account) => Ok(serde_json::to_string(&account).unwrap()),
        Err(e) => Err(OtherError(e).rejection()),
    }
}

pub async fn change_name_handler(
    session_key: String,
    request_body: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    match crate::session::get_account_by_session(&session_key) {
        Ok(account) => {
            let changed_name = request_body
                .get(&"changed_name".to_string())
                .ok_or(ChangedNameNotFound.rejection())?;
            let repos = MySQLClient::new();
            let mut new = account.clone();
            new.set_name(changed_name.clone());
            repos
                .rename_account(&new)
                .map_err(|e| OtherError(e).rejection())?;
            Ok(serde_json::to_string(&new).unwrap())
        }
        Err(e) => Err(OtherError(e).rejection()),
    }
}

pub async fn logout_handler(session_key: String) -> Result<impl Reply, Rejection> {
    crate::session::remove_session(&session_key)
        .map_err(|e| HandleError::OtherError(e).rejection())?;
    Ok(StatusCode::OK)
}

pub async fn oauth(query: HashMap<String, String>) -> Result<impl Reply, Rejection> {
    let code = query
        .get(&"code".to_string())
        .cloned()
        .ok_or(HandleError::AuthorizationCodeIsNotFound.rejection())?;
    let profile = oauth_google::verify(code)
        .await
        .map_err(|e| HandleError::OAuthGoogleError(e).rejection())?;
    dbg!(&profile);
    let repos = MySQLClient::new();
    let account = repos
        .register(&profile)
        .map_err(|_| HandleError::AccountIsNotFound.rejection())?;
    let key =
        save_user_id(account.google_id).map_err(|e| HandleError::OtherError(e).rejection())?;
    let header = format!(
        "session-token={};domain={};max-age=300",
        key,
        config().client_domain
    );

    let uri = Uri::from_maybe_shared(config().client_url).unwrap();
    let redirect = warp::redirect(uri);
    Ok(warp::reply::with_header(
        redirect,
        http::header::SET_COOKIE,
        header,
    ))
}

fn date(map: &HashMap<String, String>) -> UpdatedAt {
    if let Some(date) = map.get("date".into()) {
        UpdatedAt::from_str(date).sub(-1)
    } else {
        UpdatedAt::new()
    }
}
