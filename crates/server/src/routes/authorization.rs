use crate::config::config;
use crate::error::HandleError;
use crate::filter::with_db;
use chrono::Duration;
use model::GoogleId;
use mysql::MySqlPool;
use oauth_google::{GoogleProfile, RegisterUser};
use repository::AccountByGoogleId;
use std::collections::HashMap;
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
        .and(warp::query::<HashMap<String, String>>().and_then(verify))
        .and_then(oauth_handler)
        .boxed()
}
const EXPIRE_DAYS: i64 = 30;

async fn oauth_handler<C: RegisterUser + AccountByGoogleId>(
    mut repos: C,
    profile: GoogleProfile,
) -> Result<impl Reply, Rejection> {
    repos
        .register(&profile)
        .await
        .map_err(HandleError::OtherError)?;
    let account = repos
        .user(&GoogleId::new(profile.user_id))
        .await
        .map_err(HandleError::OtherError)?;
    let key = session::generate_session_jwt(account.user_id, Duration::days(EXPIRE_DAYS))
        .map_err(HandleError::OtherError)?;

    let header = format!(
        "session-token={};domain={};max-age=2592000",
        key,
        config().client_domain
    );

    let uri = Uri::from_maybe_shared(config().client_url.clone()).unwrap();
    let redirect = warp::redirect(uri);
    Ok(warp::reply::with_header(
        redirect,
        warp::http::header::SET_COOKIE,
        header,
    ))
}

async fn verify(query: HashMap<String, String>) -> Result<GoogleProfile, Rejection> {
    let code = query
        .get("code")
        .cloned()
        .ok_or(HandleError::AuthorizationCodeIsNotFound)?;
    let profile = oauth_google::verify(code)
        .await
        .map_err(HandleError::OAuthGoogleError)?;
    Ok(profile)
}
