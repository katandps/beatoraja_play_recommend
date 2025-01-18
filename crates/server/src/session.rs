use anyhow::Result;
use chrono::Duration;
use model::*;
use repository::AccountByUserId;
use warp::Rejection;

use crate::error::HandleError;

pub const SESSION_KEY: &str = "session-token";
const EXPIRE_DAYS: i64 = 30;

pub fn save_user_id(user_id: UserId) -> Result<String> {
    session::generate_session_jwt(user_id, Duration::days(EXPIRE_DAYS))
}

pub async fn get_account_by_session<C: AccountByUserId>(
    mut repos: C,
    jwt: String,
) -> Result<Account, Rejection> {
    let claims = session::verify_session_jwt(&jwt).map_err(HandleError::from)?;
    Ok(repos
        .user(claims.user_id.get())
        .await
        .map_err(HandleError::from)?)
}
