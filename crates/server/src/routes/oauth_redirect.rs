use crate::config::config;
use crate::error::HandleError;
use crate::filter::with_db;
use model::GoogleId;
use mysql::MySqlPool;
use oauth_google::{GoogleProfile, RegisterUser};
use repository::AccountByGoogleId;
use std::collections::HashMap;
use warp::filters::BoxedFilter;
use warp::http::Uri;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("oauth"))
        .and(with_db(db_pool))
        .and(warp::query::<HashMap<String, String>>().and_then(verify))
        .and_then(oauth_handler)
        .boxed()
}

async fn oauth_handler<C: RegisterUser + AccountByGoogleId>(
    repos: C,
    profile: GoogleProfile,
) -> Result<impl Reply, Rejection> {
    repos.register(&profile).map_err(HandleError::OtherError)?;
    let account = repos
        .user(&GoogleId::new(profile.user_id))
        .map_err(HandleError::OtherError)?;
    let key = crate::session::save_user_id(account.google_id).map_err(HandleError::OtherError)?;
    let header = format!(
        "session-token={};domain={};max-age=300",
        key,
        config().client_domain
    );

    let uri = Uri::from_maybe_shared(config().client_url).unwrap();
    let redirect = warp::redirect(uri);
    Ok(warp::reply::with_header(
        redirect,
        http::header::SET_COOKIE,
        header,
    ))
}

async fn verify(query: HashMap<String, String>) -> Result<GoogleProfile, Rejection> {
    let code = query
        .get(&"code".to_string())
        .cloned()
        .ok_or(HandleError::AuthorizationCodeIsNotFound)?;
    let profile = oauth_google::verify(code)
        .await
        .map_err(HandleError::OAuthGoogleError)?;
    Ok(profile)
}
