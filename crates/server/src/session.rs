use crate::config::config;
use crate::error::HandleError;
use crate::error::HandleError::SessionError;
use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use model::*;
use redis::{Commands, Connection, RedisResult};
use repository::AccountByGoogleId;
use serde::{Deserialize, Serialize};
use warp::Rejection;

pub const SESSION_KEY: &str = "session-token";
const EXPIRE_SECONDS: u64 = 30 * 24 * 60 * 60;

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

pub fn remove_session(key: &str) -> Result<()> {
    let mut redis_connection = get_client()?;
    let _: () = redis_connection.del(key)?;
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: UserId,
    exp: i64,
    admin: bool,
}

#[allow(unused)]
fn generate_session_jwt(user_id: UserId) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(30))
        .expect("invalid timestamp");
    let claims = Claims {
        user_id,
        exp: expiration.timestamp(),
        admin: false,
    };
    let token = encode(
        &Header::new(jsonwebtoken::Algorithm::ES256),
        &claims,
        &get_private_key()?,
    )?;
    Ok(token)
}

#[allow(unused)]
fn verify_session_jwt(jwt: &str) -> Result<Claims> {
    let decoded = decode::<Claims>(
        jwt,
        &get_public_key()?,
        &Validation::new(jsonwebtoken::Algorithm::ES256),
    )?;
    Ok(decoded.claims)
}

#[allow(unused)]
fn get_private_key() -> Result<EncodingKey> {
    let private_key = std::env::var("SESSION_JWT_PRIVATE_KEY").unwrap();
    let result = EncodingKey::from_ec_pem(&private_key.into_bytes())?;
    Ok(result)
}

#[allow(unused)]
fn get_public_key() -> Result<DecodingKey> {
    let public_key = std::env::var("SESSION_JWT_PUBLIC_KEY").unwrap();
    let result = DecodingKey::from_ec_pem(&public_key.into_bytes())?;
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let data = [1u8, 2, 3, 4, 5, 6, 7, 8, 123];
        assert_eq!("12345678123".to_string(), join(&data))
    }

    #[test]
    fn generate() {
        std::env::set_var(
            "SESSION_JWT_PUBLIC_KEY",
            r"-----BEGIN PUBLIC KEY-----
MEkwEwYHKoZIzj0CAQYIKoZIzj0DAQEDMgAE0JLtHjBYATwilJVTv4lEfxx28tV2
7hhUA77sYd1UBJfGAnCpPOOlc9RuhexDUl/W
-----END PUBLIC KEY-----
",
        );
        std::env::set_var(
            "SESSION_JWT_PRIVATE_KEY",
            r"-----BEGIN EC PRIVATE KEY-----
MF8CAQEEGKqdNdt+XWOEROL5eNNo8lL/vgl20yJwp6AKBggqhkjOPQMBAaE0AzIA
BNCS7R4wWAE8IpSVU7+JRH8cdvLVdu4YVAO+7GHdVASXxgJwqTzjpXPUboXsQ1Jf
1g==
-----END EC PRIVATE KEY-----
",
        );
        let user_id = 1;
        let jwt = generate_session_jwt(UserId::new(user_id));
        dbg!(&jwt);
        let claims = verify_session_jwt(&jwt.unwrap());
        dbg!(&claims);
    }
}
