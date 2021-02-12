use crate::config::config;
use crate::error::HandleError;
use model::GoogleId;
use oauth_google::{GoogleProfile, RegisterUser};
use repository::AccountByGoogleId;
use warp::http::Uri;
use warp::{Rejection, Reply};

pub async fn oauth_handler<C: RegisterUser + AccountByGoogleId>(
    repos: C,
    profile: GoogleProfile,
) -> Result<impl Reply, Rejection> {
    repos
        .register(&profile)
        .map_err(|e| HandleError::OtherError(e))?;
    let account = repos
        .user(&GoogleId::new(profile.user_id))
        .map_err(|e| HandleError::OtherError(e))?;
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
