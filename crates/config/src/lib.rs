#[macro_use]
extern crate lazy_static;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Cfg {
    #[serde(default = "redis_url")]
    pub redis_url: String,

    #[serde(default = "google_oauth_client_id")]
    pub google_oauth_client_id: String,
    #[serde(default = "google_oauth_client_secret")]
    pub google_oauth_client_secret: String,
    #[serde(default = "google_oauth_redirect_uri")]
    pub google_oauth_redirect_uri: String,
    #[serde(default = "client_url")]
    pub client_url: String,
    #[serde(default = "client_domain")]
    pub client_domain: String,
    #[serde(default = "mysql_url")]
    pub mysql_url: String,
}

fn mysql_url() -> String {
    "mysql://root:root@mysql:3306/user_data".into()
}

fn redis_url() -> String {
    "redis://session-redis:6379".into()
}

fn client_url() -> String {
    "http://localhost:8080".into()
}

fn client_domain() -> String {
    "localhost".into()
}

fn google_oauth_client_id() -> String {
    "746230605395-pc3t46mk87koas61js1k2uu87g2d3q5g.apps.googleusercontent.com".into()
}

fn google_oauth_client_secret() -> String {
    "client secret".into()
}

fn google_oauth_redirect_uri() -> String {
    "https://localhost:4431/oauth".into()
}

pub fn config() -> Cfg {
    (*self::CONFIG).clone()
}

lazy_static! {
    pub static ref CONFIG: Cfg = {
        match envy::prefixed("APP_").from_env::<Cfg>() {
            Ok(val) => val,
            Err(err) => {
                println!("{}", err);
                std::process::exit(1)
            }
        }
    };
}
