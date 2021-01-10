use http::StatusCode;
use warp::{Rejection, Reply};

pub async fn logout_handler(session_key: String) -> Result<impl Reply, Rejection> {
    crate::session::remove_session(&session_key)?;
    Ok(StatusCode::OK)
}
