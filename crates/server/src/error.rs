use crate::error::HandleError::{IOError, MySqlError, OtherError, RedisError, WarpError};
use serde_derive::Serialize;
use std::convert::Infallible;
use std::num::ParseIntError;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Debug, Error)]
pub enum HandleError {
    #[error("Code Is Not Found")]
    AuthorizationCodeIsNotFound,

    #[error("Account Not Found: {0:?}")]
    AccountIsNotFound(anyhow::Error),
    #[error("Account Is Not Selected")]
    AccountIsNotSelected,
    #[error("Account Selection Is Invalid")]
    AccountSelectionIsInvalid(ParseIntError),

    #[error("IOError: {0:?}")]
    IOError(std::io::Error),
    #[error("MySqlError: {0:?}")]
    MySqlError(mysql::Error),

    #[error("WarpError: {0:?}")]
    WarpError(warp::Error),

    #[error("Upload Failed")]
    DirectoryCouldNotCreated,
    #[error("File Is Not Found")]
    FileIsNotFound,
    #[error("File Is Not Deleted")]
    FileIsNotDeleted,
    #[error("Save Is Not Complete")]
    SaveIsNotComplete,

    #[error("Changed Name Is Not Found")]
    ChangedNameNotFound,

    #[error("OAuthGoogleError: {0:?}")]
    OAuthGoogleError(oauth_google::Error),
    #[error("Other Error: {0}")]
    OtherError(anyhow::Error),
    #[error("RedisError: {0:?}")]
    RedisError(redis::RedisError),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
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
                FileIsNotFound => StatusCode::OK,
                SaveIsNotComplete => StatusCode::OK,
                FileIsNotDeleted => StatusCode::OK,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            e.to_string(),
        )
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        (StatusCode::BAD_REQUEST, "Invalid Body".into())
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        (StatusCode::UNAUTHORIZED, "Method Not Allowed".into())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".into(),
        )
    };

    println!("{} {}", code, message);
    let json = warp::reply::json(&ErrorResponse {
        error: message.into(),
    });
    Ok(warp::reply::with_status(json, code))
}

impl HandleError {
    pub fn rejection(self) -> Rejection {
        warp::reject::custom(self)
    }
}

impl warp::reject::Reject for HandleError {}

impl From<HandleError> for Rejection {
    fn from(e: HandleError) -> Self {
        warp::reject::custom(e)
    }
}

impl From<anyhow::Error> for HandleError {
    fn from(e: anyhow::Error) -> Self {
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

impl From<redis::RedisError> for HandleError {
    fn from(e: redis::RedisError) -> Self {
        RedisError(e)
    }
}

impl From<mysql::Error> for HandleError {
    fn from(e: mysql::Error) -> Self {
        MySqlError(e)
    }
}
