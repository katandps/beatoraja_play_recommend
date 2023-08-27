use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Clone, Debug, Deserialize)]
pub struct Cfg {
    #[serde(default = "mysql_url")]
    pub mysql_url: String,
}

fn mysql_url() -> String {
    "mysql://root:root@mysql:3306/user_data".into()
}

pub fn config() -> &'static Cfg {
    static INSTANCE: OnceLock<Cfg> = OnceLock::new();
    INSTANCE.get_or_init(|| envy::from_env::<Cfg>().unwrap())
}
