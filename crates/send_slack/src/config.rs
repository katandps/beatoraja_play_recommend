use serde::Deserialize;

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

pub fn config() -> Cfg {
    (*self::CONFIG).clone()
}

lazy_static! {
    pub static ref CONFIG: Cfg = {
        match envy::prefixed("APP_").from_env::<Cfg>() {
            Ok(val) => val,
            Err(e) => {
                panic!(format!("{}", e))
            }
        }
    };
}
