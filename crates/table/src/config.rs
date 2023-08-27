use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Clone, Debug, Deserialize)]
pub struct Cfg {
    #[serde(default = "default_table_urls")]
    pub table_urls: Vec<String>,
}

fn default_table_urls() -> Vec<String> {
    [
        "http://www.ribbit.xyz/bms/tables/insane.html",
        "http://www.ribbit.xyz/bms/tables/overjoy.html",
        "https://stellabms.xyz/st/table.html",
        "https://stellabms.xyz/sl/table.html",
        "http://flowermaster.web.fc2.com/lrnanido/gla/LN.html",
        "http://rattoto10.jounin.jp/table_insane.html",
        "http://rattoto10.jounin.jp/table.html",
        "http://walkure.net/hakkyou/for_glassist/bms/?lamp=easy",
    ]
    .iter()
    .map(|&s| s.into())
    .collect()
}

pub fn config() -> &'static Cfg {
    static INSTANCE: OnceLock<Cfg> = OnceLock::new();
    INSTANCE.get_or_init(|| envy::from_env::<Cfg>().unwrap())
}
