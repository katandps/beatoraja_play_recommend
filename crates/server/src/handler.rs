pub mod detail;
pub mod health;
pub mod upload;

use crate::error::HandleError::{
    ChangedNameNotFound, FromUtf8Error, GoogleResponseIsInvalid, OtherError,
};
use crate::error::*;
use crate::session::save_user_id;
use config::config;
use http::StatusCode;
use model::*;
use mysql::MySQLClient;
use sqlite::SqliteClient;
use std::collections::HashMap;
use warp::http::Uri;
use warp::{Rejection, Reply};

pub async fn table_handler(tables: Tables) -> std::result::Result<impl Reply, Rejection> {
    Ok(serde_json::to_string(&tables.format()).unwrap())
}

pub async fn history_handler() -> std::result::Result<impl Reply, Rejection> {
    let repos = SqliteClient::by_config();
    Ok(serde_json::to_string(&repos.player().diff()).unwrap())
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
    let mut body = HashMap::new();
    body.insert("client_id", config().google_oauth_client_id());
    body.insert("client_secret", config().google_oauth_client_secret());
    body.insert("redirect_uri", config().google_oauth_redirect_uri());
    body.insert("code", code.clone());
    body.insert("grant_type", "authorization_code".to_string());
    let res = reqwest::Client::new()
        .post("https://accounts.google.com/o/oauth2/token")
        .json(&body)
        .send()
        .await
        .map_err(|e| HandleError::ReqwestError(e).rejection())?;
    let body = res
        .text()
        .await
        .map_err(|e| HandleError::ReqwestError(e).rejection())?;
    let json: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| HandleError::SerdeJsonError(e).rejection())?;
    let obj = json.as_object().unwrap();

    let token = &obj
        .get(&"id_token".to_string())
        .ok_or(GoogleResponseIsInvalid.rejection())?
        .to_string()
        .replace("\"", "")
        .replace(",", "");
    let mut segments = token.split('.');
    let _encoded_header = segments.next().ok_or(GoogleResponseIsInvalid.rejection())?;
    let encoded_payload = segments.next().ok_or(GoogleResponseIsInvalid.rejection())?;

    let payload_string = String::from_utf8(
        base64::decode_config(&encoded_payload, base64::URL_SAFE_NO_PAD).unwrap(),
    )
    .map_err(|e| FromUtf8Error(e).rejection())?;
    let payload_json: serde_json::Value =
        serde_json::from_str::<serde_json::Value>(&payload_string)
            .map_err(|e| HandleError::SerdeJsonError(e).rejection())?;
    let payload = payload_json
        .as_object()
        .ok_or(HandleError::GoogleResponseIsInvalid.rejection())?;

    let user_id = payload
        .get(&"sub".to_string())
        .ok_or(GoogleResponseIsInvalid.rejection())?
        .to_string()
        .replace("\"", "");
    let email = payload
        .get(&"email".to_string())
        .ok_or(GoogleResponseIsInvalid.rejection())?
        .to_string()
        .replace("\"", "");
    let name = "default_name".to_string();
    let profile = GoogleProfile {
        user_id,
        email,
        name,
    };
    println!(
        "Login: {} {} {}",
        profile.user_id, profile.name, profile.email
    );
    let repos = MySQLClient::new();
    let account = repos
        .register(&profile)
        .map_err(|_| HandleError::AccountIsNotFound.rejection())?;
    let key =
        save_user_id(account.google_id).map_err(|e| HandleError::OtherError(e).rejection())?;
    let header = format!(
        "session-token={};domain={};max-age=300",
        key,
        config().client_domain()
    );

    let uri = Uri::from_maybe_shared(format!("{}", config().client_url())).unwrap();
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
