use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Clone, Debug, Deserialize)]
pub struct Cfg {
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
}
fn client_url() -> String {
    "http://localhost:8080".into()
}

fn client_domain() -> String {
    "localhost".into()
}

fn google_oauth_client_id() -> String {
    "hogehogehogehoge.apps.googleusercontent.com".into()
}

fn google_oauth_client_secret() -> String {
    "client secret".into()
}

fn google_oauth_redirect_uri() -> String {
    "https://localhost:4431/oauth".into()
}

pub fn config() -> &'static Cfg {
    static INSTANCE: OnceLock<Cfg> = OnceLock::new();
    INSTANCE.get_or_init(|| envy::from_env::<Cfg>().unwrap())
}
