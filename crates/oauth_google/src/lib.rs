mod config;
mod error;

use anyhow::Result;
use base64::{alphabet, engine, read};
use config::config;
pub use error::Error;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::HashMap;

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
    body.insert("client_id", config().google_oauth_client_id.clone());
    body.insert("client_secret", config().google_oauth_client_secret.clone());
    body.insert("redirect_uri", config().google_oauth_redirect_uri.clone());
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

    let json: Value = serde_json::from_str(&body).map_err(Error::SerdeJsonError)?;
    Ok(json.as_object().unwrap().clone())
}

fn get_payload(obj: &Map<String, Value>) -> Result<Map<String, Value>, Error> {
    let token = &obj
        .get(&"id_token".to_string())
        .ok_or_else(|| Error::GoogleResponseIsInvalid("id_token is not found".into()))?
        .to_string()
        .replace(['\"', ','], "");
    let mut segments = token.split('.');
    let _encoded_header = segments
        .next()
        .ok_or_else(|| Error::GoogleResponseIsInvalid("could not get first segment".into()))?;
    let encoded_payload = segments
        .next()
        .ok_or_else(|| Error::GoogleResponseIsInvalid("could not get second segment".into()))?;

    let base64_engine =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, engine::general_purpose::NO_PAD);
    let payload_string = read::DecoderReader::new(encoded_payload.as_bytes(), &base64_engine);
    // .map_err(|_| Error::GoogleResponseIsInvalid("payload is not encoded in base64".into()))?;
    let payload_json: Value =
        serde_json::from_reader(payload_string).map_err(Error::SerdeJsonError)?;
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
    fn register(
        &mut self,
        profile: &GoogleProfile,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

#[derive(Deserialize)]
pub struct AuthorizationQuery {
    pub code: String,
}

impl AuthorizationQuery {
    pub async fn get_profile(self) -> Result<GoogleProfile, Error> {
        let body = make_token_request_body(self.code);
        let obj = token_request(body).await?;
        let payload = get_payload(&obj)?;
        make_google_profile(&payload)
    }
}
