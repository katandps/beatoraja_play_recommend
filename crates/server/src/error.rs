use crate::error::HandleError::*;
use serde::Serialize;
use std::convert::Infallible;
use std::num::ParseIntError;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Debug, Error)]
pub enum HandleError {
    #[error("AuthorizationCodeIsNotFound")]
    AuthorizationCodeIsNotFound,

    #[error("AccountIsNotFound: {0:?}")]
    AccountIsNotFound(anyhow::Error),
    #[error("AccountIsNotSelected")]
    AccountIsNotSelected,
    #[error("AccountSelectionIsInvalid")]
    AccountSelectionIsInvalid(ParseIntError),

    #[error("IOError: {0:?}")]
    IOError(std::io::Error),
    #[error("MySqlError: {0:?}")]
    MySqlError(mysql::Error),
    #[error("SqliteError: {0:?}")]
    SqliteError(sqlite::SqliteError),

    #[error("WarpError: {0:?}")]
    WarpError(warp::Error),
    #[error("FormIsIncomplete")]
    FormIsIncomplete,

    #[error("ChangedNameNotFound")]
    ChangedNameNotFound,
    #[error("ChangedVisibilityNotFound")]
    ChangedVisibilityNotFound,

    #[error("OAuthGoogleError: {0:?}")]
    OAuthGoogleError(oauth_google::Error),
    #[error("OtherError: {0:?}")]
    OtherError(anyhow::Error),
    #[error("SerdeJsonError: {0:?}")]
    SerdeJsonError(serde_json::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message): (StatusCode, String) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".into())
    } else if let Some(e) = err.find::<HandleError>() {
        use HandleError::*;
        (
            match e {
                AuthorizationCodeIsNotFound => StatusCode::BAD_REQUEST,
                AccountIsNotFound(_) => StatusCode::BAD_REQUEST,
                AccountIsNotSelected => StatusCode::BAD_REQUEST,
                AccountSelectionIsInvalid(_) => StatusCode::BAD_REQUEST,
                WarpError(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            e.to_string(),
        )
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        (StatusCode::BAD_REQUEST, "Invalid Body".into())
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::UNAUTHORIZED, "Method Not Allowed".into())
    } else {
        log::error!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".into(),
        )
    };

    log::error!("{} {}", code, message);
    let json = warp::reply::json(&ErrorResponse { error: message });
    Ok(warp::reply::with_status(json, code))
}

impl warp::reject::Reject for HandleError {}

impl From<anyhow::Error> for HandleError {
    fn from(e: anyhow::Error) -> Self {
        log::warn!("{:?}", e);
        OtherError(e)
    }
}

impl From<std::io::Error> for HandleError {
    fn from(e: std::io::Error) -> Self {
        IOError(e)
    }
}

impl From<warp::Error> for HandleError {
    fn from(e: warp::Error) -> Self {
        WarpError(e)
    }
}

impl From<mysql::Error> for HandleError {
    fn from(e: mysql::Error) -> Self {
        MySqlError(e)
    }
}

impl From<sqlite::SqliteError> for HandleError {
    fn from(e: sqlite::SqliteError) -> Self {
        SqliteError(e)
    }
}

impl From<serde_json::Error> for HandleError {
    fn from(e: serde_json::Error) -> Self {
        SerdeJsonError(e)
    }
}
