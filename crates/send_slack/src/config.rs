use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Clone, Debug, Deserialize)]
pub struct Cfg {
    #[serde(default)]
    pub slack_bot_token: String,
    #[serde(default)]
    pub slack_channel: String,
    #[serde(default)]
    pub slack_file_name: String,
}

pub fn config() -> &'static Cfg {
    static INSTANCE: OnceLock<Cfg> = OnceLock::new();
    INSTANCE.get_or_init(|| envy::from_env::<Cfg>().unwrap())
}
