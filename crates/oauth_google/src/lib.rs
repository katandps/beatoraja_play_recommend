mod config;
mod error;

#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use config::config;
pub use error::Error;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub async fn verify(code: String) -> Result<GoogleProfile, Error> {
    let body = make_token_request_body(code);
    let obj = token_request(body).await?;
    let payload = get_payload(&obj)?;
    make_google_profile(&payload)
}

fn make_google_profile(payload: &Map<String, Value>) -> Result<GoogleProfile, Error> {
    let user_id = payload
        .get(&"sub".to_string())
        .ok_or_else(|| Error::GoogleResponseIsInvalid("subject is not found in payload".into()))?
        .to_string()
        .replace('\"', "");
    let email = payload
        .get(&"email".to_string())
        .ok_or_else(|| Error::GoogleResponseIsInvalid("email is not found in payload".into()))?
        .to_string()
        .replace('\"', "");
    let name = "default_name".to_string();
    log::info!("{} {}", user_id, email);
    Ok(GoogleProfile {
        user_id,
        email,
        name,
    })
}

fn make_token_request_body(code: String) -> HashMap<&'static str, String> {
    let mut body = HashMap::new();
    body.insert("client_id", config().google_oauth_client_id);
    body.insert("client_secret", config().google_oauth_client_secret);
    body.insert("redirect_uri", config().google_oauth_redirect_uri);
    body.insert("code", code);
    body.insert("grant_type", "authorization_code".to_string());
    body
}

async fn token_request(body: HashMap<&str, String>) -> Result<Map<String, Value>, Error> {
    let res = reqwest::Client::new()
        .post("https://accounts.google.com/o/oauth2/token")
        .json(&body)
        .send()
        .await
        .map_err(Error::ReqwestError)?;
    let body = res.text().await.map_err(Error::ReqwestError)?;

    let json: serde_json::Value = serde_json::from_str(&body).map_err(Error::SerdeJsonError)?;
    Ok(json.as_object().unwrap().clone())
}

fn get_payload(obj: &Map<String, Value>) -> Result<Map<String, Value>, Error> {
    let token = &obj
        .get(&"id_token".to_string())
        .ok_or_else(|| Error::GoogleResponseIsInvalid("id_token is not found".into()))?
        .to_string()
        .replace('\"', "")
        .replace(',', "");

    let mut segments = token.split('.');
    let _encoded_header = segments
        .next()
        .ok_or_else(|| Error::GoogleResponseIsInvalid("could not get first segment".into()))?;
    let encoded_payload = segments
        .next()
        .ok_or_else(|| Error::GoogleResponseIsInvalid("could not get second segment".into()))?;

    let payload_string = String::from_utf8(
        base64::decode_config(&encoded_payload, base64::URL_SAFE_NO_PAD).map_err(|_| {
            Error::GoogleResponseIsInvalid("payload is not encoded in base64 ".into())
        })?,
    )
    .map_err(Error::FromUtf8Error)?;
    let payload_json: serde_json::Value =
        serde_json::from_str::<serde_json::Value>(&payload_string)
            .map_err(Error::SerdeJsonError)?;
    Ok(payload_json
        .as_object()
        .ok_or_else(|| Error::GoogleResponseIsInvalid("second segment is invalid".into()))?
        .clone())
}

#[derive(Clone, Debug)]
pub struct GoogleProfile {
    pub user_id: String,
    pub email: String,
    pub name: String,
}

pub trait RegisterUser {
    fn register(&mut self, profile: &GoogleProfile) -> Result<()>;
}
