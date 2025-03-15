use anyhow::Result;
use chrono::Duration;
use model::GoogleId;
use oauth_google::{AuthorizationQuery, RegisterUser};
use repository::AccountByGoogleId;

const EXPIRE_DAYS: i64 = 30;

pub async fn register<C: RegisterUser + AccountByGoogleId>(
    mut repos: C,
    query: AuthorizationQuery,
) -> Result<Registered> {
    let profile = query.get_profile().await?;
    repos.register(&profile).await?;
    let account = repos.user(&GoogleId::new(profile.user_id)).await?;
    let session_period = Duration::days(EXPIRE_DAYS);
    let session_key = session::generate_session_jwt(account.user_id, session_period)?;
    Ok(Registered {
        session_key,
        session_period,
    })
}

pub struct Registered {
    pub session_key: String,
    pub session_period: Duration,
}
