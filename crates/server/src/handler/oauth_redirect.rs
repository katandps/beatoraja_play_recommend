use crate::config::config;
use crate::error::HandleError;
use model::GoogleId;
use mysql::MySQLClient;
use oauth_google::{GoogleProfile, RegisterUser};
use warp::http::Uri;
use warp::{Rejection, Reply};

pub async fn oauth_handler(
    repos: MySQLClient,
    profile: GoogleProfile,
) -> Result<impl Reply, Rejection> {
    repos
        .register(&profile)
        .map_err(|e| HandleError::OtherError(e))?;
    let account = repos
        .account_by_id(&GoogleId::new(profile.user_id))
        .map_err(|e| HandleError::MySqlError(e))?;
    let key =
        crate::session::save_user_id(account.google_id).map_err(|e| HandleError::OtherError(e))?;
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
