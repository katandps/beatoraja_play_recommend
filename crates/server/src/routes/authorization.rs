use crate::config::config;
use crate::filter::with_db;
use mysql::MySqlPool;
use service::authorization::Registered;
use warp::filters::BoxedFilter;
use warp::http::StatusCode;
use warp::http::Uri;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn routes(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    oauth_redirect(db_pool)
        .or(logout())
        .with(warp::compression::gzip())
        .boxed()
}

pub fn logout() -> BoxedFilter<(impl Reply,)> {
    async fn logout_handler() -> Result<impl Reply, Rejection> {
        let header = format!("session-token=;domain={};max-age=0", config().client_domain);
        Ok(warp::reply::with_header(
            StatusCode::OK,
            warp::http::header::SET_COOKIE,
            header,
        ))
    }
    warp::get()
        .and(path("logout"))
        .and_then(logout_handler)
        .boxed()
}

pub fn oauth_redirect(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("oauth"))
        .and(with_db(db_pool))
        .and(warp::query())
        .then(service::authorization::register)
        .then(redirect)
        .boxed()
}

pub async fn redirect(result: anyhow::Result<Registered>) -> impl Reply {
    match result {
        Ok(Registered {
            session_key,
            session_period,
        }) => {
            let header = format!(
                "session-token={};domain={};max-age={}",
                session_key,
                config().client_domain,
                session_period.num_seconds()
            );
            let uri = Uri::from_maybe_shared(config().client_url.clone()).unwrap();
            let redirect = warp::redirect(uri);
            warp::reply::with_header(redirect, warp::http::header::SET_COOKIE, header)
        }

        Err(e) => {
            log::error!("registration error: {:?}", e);
            panic!();
        }
    }
}
