use crate::config::config;
use warp::filters::BoxedFilter;
use warp::http::StatusCode;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn route() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("logout"))
        .and_then(logout_handler)
        .boxed()
}

async fn logout_handler() -> Result<impl Reply, Rejection> {
    let header = format!("session-token=;domain={};max-age=0", config().client_domain);
    Ok(warp::reply::with_header(
        StatusCode::OK,
        warp::http::header::SET_COOKIE,
        header,
    ))
}
