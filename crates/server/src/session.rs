use crate::config::config;
use crate::error::HandleError;
use anyhow::Result;
use model::*;
use mysql::MySQLClient;
use redis::{Commands, Connection, RedisResult};

pub const SESSION_KEY: &str = "session-token";
const EXPIRE_SECONDS: usize = 2 * 60 * 60;

pub fn get_client() -> RedisResult<Connection> {
    let client = redis::Client::open(config().redis_url)?;
    client.get_connection()
}

pub fn save_user_id(user_id: GoogleId) -> Result<String> {
    let mut redis_connection = get_client()?;
    let key = generate_session_key();
    let _: String = redis_connection.set_ex(key.clone(), user_id.to_string(), EXPIRE_SECONDS)?;
    Ok(key)
}

pub fn remove_session(key: &String) -> Result<(), HandleError> {
    let mut redis_connection = get_client()?;
    let _ = redis_connection.del(key)?;
    Ok(())
}

pub fn get_user_id(key: &String) -> Result<GoogleId, HandleError> {
    let mut redis_connection = get_client()?;
    Ok(GoogleId::new(redis_connection.get(key)?))
}

pub fn get_account(repos: &MySQLClient, user_id: GoogleId) -> Result<Account, HandleError> {
    Ok(repos.account_by_id(user_id)?)
}

pub fn get_account_by_session(repos: &MySQLClient, key: &String) -> Result<Account, HandleError> {
    let user_id = get_user_id(key)?;
    Ok(get_account(repos, user_id)?)
}

fn generate_session_key() -> String {
    use bigint::U256;
    use rand::prelude::*;
    use rand_chacha::ChaCha20Rng;

    let mut csp_rng = ChaCha20Rng::from_entropy();
    let mut data = [0u8; 32];
    csp_rng.fill_bytes(&mut data);
    format!("{}", U256::from(data))
}
