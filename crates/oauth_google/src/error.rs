use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("FromUtf8Error: {0:?}")]
    FromUtf8Error(FromUtf8Error),
    #[error("SerdeJsonError: {0:?}")]
    SerdeJsonError(serde_json::Error),
    #[error("GoogleResponseIsInvalid: {0}")]
    GoogleResponseIsInvalid(String),
    #[error("ReqwestError: {0:?}")]
    ReqwestError(reqwest::Error),
}
