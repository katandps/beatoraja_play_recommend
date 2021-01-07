use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Cfg {
    #[serde(default = "redis_url")]
    pub redis_url: String,
    #[serde(default = "client_url")]
    pub client_url: String,
    #[serde(default = "client_domain")]
    pub client_domain: String,
    #[serde(default = "tls_cert_path")]
    pub tls_cert_path: String,
    #[serde(default = "tls_key_path")]
    pub tls_key_path: String,
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

fn tls_cert_path() -> String {
    "./files/cert.pem".into()
}

fn tls_key_path() -> String {
    "./files/key.rsa".into()
}

pub fn config() -> Cfg {
    (*self::CONFIG).clone()
}

lazy_static! {
    pub static ref CONFIG: Cfg = {
        match envy::from_env::<Cfg>() {
            Ok(val) => val,
            Err(e) => {
                panic!(format!("{}", e))
            }
        }
    };
}
