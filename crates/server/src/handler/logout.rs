use crate::filter::receive_session_key;
use http::StatusCode;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn logout() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("logout"))
        .and(receive_session_key())
        .and_then(logout_handler)
        .boxed()
}

async fn logout_handler(session_key: String) -> Result<impl Reply, Rejection> {
    crate::session::remove_session(&session_key)?;
    Ok(StatusCode::OK)
}
