use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Clone, Debug, Deserialize)]
pub struct Cfg {
    #[serde(default = "slack_bot_token")]
    pub slack_bot_token: String,
    #[serde(default = "slack_channel")]
    pub slack_channel: String,
    #[serde(default = "slack_file_name")]
    pub slack_file_name: String,
}

fn slack_bot_token() -> String {
    "".into()
}

fn slack_channel() -> String {
    "".into()
}

fn slack_file_name() -> String {
    "".into()
}

pub fn config() -> &'static Cfg {
    static INSTANCE: OnceLock<Cfg> = OnceLock::new();
    INSTANCE.get_or_init(|| envy::from_env::<Cfg>().unwrap())
}
