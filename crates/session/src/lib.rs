use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use model::*;
use serde::{Deserialize, Serialize};

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
