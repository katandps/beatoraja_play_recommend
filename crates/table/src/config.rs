use model::TableId;
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Clone, Debug, Deserialize)]
pub struct Cfg {
    pub tables: Vec<TableSetting>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TableSetting {
    pub id: TableId,
    pub title: String,
    pub url: String,
}

pub fn config() -> &'static Cfg {
    static INSTANCE: OnceLock<Cfg> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        const DEFAULT_SETTING_PATH: &str = "./tables.toml";
        let toml = std::fs::read_to_string(DEFAULT_SETTING_PATH).unwrap();
        toml::from_str(&toml).unwrap()
    })
}
