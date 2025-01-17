use std::sync::OnceLock;

use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use model::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: UserId,
    exp: i64,
    pub admin: bool,
}

pub fn generate_session_jwt(user_id: UserId, period: Duration) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(period)
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

pub fn verify_session_jwt(jwt: &str) -> Result<Claims> {
    let decoded = decode::<Claims>(
        jwt,
        &get_public_key()?,
        &Validation::new(jsonwebtoken::Algorithm::ES256),
    )?;
    Ok(decoded.claims)
}

#[allow(unused)]
fn get_private_key() -> Result<EncodingKey> {
    static PRIVATE_KEY: OnceLock<String> = OnceLock::new();
    let private_key = PRIVATE_KEY.get_or_init(|| {
        if let Ok(key) = std::env::var("SESSION_JWT_PRIVATE_KEY") {
            key
        } else {
            let path: String = std::env::var("SESSION_JWT_PRIVATE_KEY_PATH").unwrap();
            std::fs::read_to_string(path).unwrap()
        }
    });
    let result = EncodingKey::from_ec_pem(private_key.as_bytes())?;
    Ok(result)
}

#[allow(unused)]
fn get_public_key() -> Result<DecodingKey> {
    static PUBLIC_KEY: OnceLock<String> = OnceLock::new();
    let public_key = PUBLIC_KEY.get_or_init(|| {
        if let Ok(key) = std::env::var("SESSION_JWT_PUBLIC_KEY") {
            key
        } else {
            let path: String = std::env::var("SESSION_JWT_PUBLIC_KEY_PATH").unwrap();
            std::fs::read_to_string(path).unwrap()
        }
    });
    let result = DecodingKey::from_ec_pem(public_key.as_bytes())?;
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate() {
        std::env::set_var(
            "SESSION_JWT_PUBLIC_KEY",
            r"-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEWU9bFwe/eMBLiLNOvBeiL1K8So6o
tnMDyo3PQgZ3QpCvuCMzo13a8ZGSraB5bx3ofZis3O6VDk42rGXNFWd5gA==
-----END PUBLIC KEY-----
",
        );
        std::env::set_var(
            "SESSION_JWT_PRIVATE_KEY",
            r"-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgy/NmLbZyP600Ic1S
+ERzBpiUd7VE8K4hynJdnaMKqwShRANCAARZT1sXB794wEuIs068F6IvUrxKjqi2
cwPKjc9CBndCkK+4IzOjXdrxkZKtoHlvHeh9mKzc7pUOTjasZc0VZ3mA
-----END PRIVATE KEY-----
",
        );
        let user_id = UserId::new(1);
        let jwt = generate_session_jwt(user_id, Duration::days(30));
        let claims = verify_session_jwt(&jwt.unwrap());
        assert_eq!(claims.unwrap().user_id, user_id);
    }
}
