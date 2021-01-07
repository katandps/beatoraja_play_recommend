use crate::config::config;
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

pub fn remove_session(key: &String) -> Result<()> {
    let mut redis_connection = get_client()?;
    let _ = redis_connection.del(key)?;
    Ok(())
}

pub fn get_user_id(key: &String) -> Result<GoogleId> {
    let mut redis_connection = get_client()?;
    Ok(GoogleId::new(redis_connection.get(key)?))
}

pub fn get_account(user_id: GoogleId) -> anyhow::Result<Account> {
    let repos = MySQLClient::new();
    repos.account_by_id(user_id)
}

pub fn get_account_by_session(key: &String) -> Result<Account> {
    let user_id = get_user_id(key)?;
    Ok(get_account(user_id)?)
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
