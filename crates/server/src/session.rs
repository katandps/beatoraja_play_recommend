use crate::config::config;
use crate::error::HandleError;
use crate::error::HandleError::SessionError;
use anyhow::Result;
use model::*;
use redis::{Commands, Connection, RedisResult};
use repository::AccountByGoogleId;
use warp::Rejection;

pub const SESSION_KEY: &str = "session-token";
const EXPIRE_SECONDS: usize = 30 * 24 * 60 * 60;

pub fn get_client() -> RedisResult<Connection> {
    let client = redis::Client::open(config().redis_url.clone())?;
    client.get_connection()
}

pub fn save_user_id(user_id: GoogleId) -> Result<String> {
    let mut redis_connection = get_client()?;
    let key = generate_session_key();
    let _: String = redis_connection.set_ex(key.clone(), user_id.to_string(), EXPIRE_SECONDS)?;
    Ok(key)
}

pub fn remove_session(key: &str) -> Result<(), HandleError> {
    let mut redis_connection = get_client()?;
    let _ = redis_connection.del(key)?;
    Ok(())
}

pub fn get_user_id(key: &str) -> Result<GoogleId, HandleError> {
    let mut redis_connection = get_client()?;
    Ok(GoogleId::new(
        redis_connection.get(key).map_err(SessionError)?,
    ))
}

pub async fn get_account<C: AccountByGoogleId>(
    mut repos: C,
    user_id: GoogleId,
) -> Result<Account, HandleError> {
    Ok(repos.user(&user_id).await?)
}

pub async fn get_account_by_session<C: AccountByGoogleId>(
    repos: C,
    key: String,
) -> Result<Account, Rejection> {
    let user_id = get_user_id(&key)?;
    Ok(get_account(repos, user_id).await?)
}

fn generate_session_key() -> String {
    use rand::prelude::*;
    use rand_chacha::ChaCha20Rng;

    let mut csp_rng = ChaCha20Rng::from_entropy();
    let mut data = [0u8; 32];
    csp_rng.fill_bytes(&mut data);
    join(&data)
}

fn join(data: &[u8]) -> String {
    data.iter().map(|u| u.to_string()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let data = [1u8, 2, 3, 4, 5, 6, 7, 8, 123];
        assert_eq!("12345678123".to_string(), join(&data))
    }
}
