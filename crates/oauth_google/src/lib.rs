mod config;
mod error;

#[macro_use]
extern crate lazy_static;

use config::config;
pub use error::Error;
use std::collections::HashMap;

pub async fn verify(code: String) -> Result<GoogleProfile, Error> {
    let mut body = HashMap::new();
    body.insert("client_id", config().google_oauth_client_id);
    body.insert("client_secret", config().google_oauth_client_secret);
    body.insert("redirect_uri", config().google_oauth_redirect_uri);
    body.insert("code", code.clone());
    body.insert("grant_type", "authorization_code".to_string());

    dbg!(&config());
    dbg!(&body);

    let res = reqwest::Client::new()
        .post("https://accounts.google.com/o/oauth2/token")
        .json(&body)
        .send()
        .await
        .map_err(|e| Error::ReqwestError(e))?;
    let body = res.text().await.map_err(|e| Error::ReqwestError(e))?;

    let json: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| Error::SerdeJsonError(e))?;
    let obj = json.as_object().unwrap();

    let token = &obj
        .get(&"id_token".to_string())
        .ok_or(Error::GoogleResponseIsInvalid(
            "id_token is not found".into(),
        ))?
        .to_string()
        .replace("\"", "")
        .replace(",", "");

    let mut segments = token.split('.');
    let _encoded_header = segments.next().ok_or(Error::GoogleResponseIsInvalid(
        "could not get first segment".into(),
    ))?;
    let encoded_payload = segments.next().ok_or(Error::GoogleResponseIsInvalid(
        "coult not get second segment".into(),
    ))?;

    let payload_string = String::from_utf8(
        base64::decode_config(&encoded_payload, base64::URL_SAFE_NO_PAD).unwrap(),
    )
    .map_err(|e| Error::FromUtf8Error(e))?;
    let payload_json: serde_json::Value =
        serde_json::from_str::<serde_json::Value>(&payload_string)
            .map_err(|e| Error::SerdeJsonError(e))?;
    let payload = payload_json
        .as_object()
        .ok_or(Error::GoogleResponseIsInvalid(
            "second segment is invalid".into(),
        ))?;

    let user_id = payload
        .get(&"sub".to_string())
        .ok_or(Error::GoogleResponseIsInvalid(
            "subject is not found in payload".into(),
        ))?
        .to_string()
        .replace("\"", "");
    let email = payload
        .get(&"email".to_string())
        .ok_or(Error::GoogleResponseIsInvalid(
            "email is not found in payload".into(),
        ))?
        .to_string()
        .replace("\"", "");
    let name = "default_name".to_string();
    Ok(GoogleProfile {
        user_id,
        email,
        name,
    })
}

#[derive(Clone, Debug)]
pub struct GoogleProfile {
    pub user_id: String,
    pub email: String,
    pub name: String,
}
