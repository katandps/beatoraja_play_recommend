use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Cfg {
    #[serde(default = "local_cache_url")]
    pub local_cache_url: String,
    #[serde(default = "default_table_urls")]
    pub table_urls: Vec<String>,
}

fn local_cache_url() -> String {
    "./files/cache.json".into()
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

pub fn config() -> Cfg {
    (*self::CONFIG).clone()
}

lazy_static! {
    pub static ref CONFIG: Cfg = {
        match envy::prefixed("").from_env::<Cfg>() {
            Ok(val) => val,
            Err(e) => {
                panic!(format!("{}", e))
            }
        }
    };
}
