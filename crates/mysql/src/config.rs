use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Cfg {
    #[serde(default = "mysql_url")]
    pub mysql_url: String,
}

fn mysql_url() -> String {
    "mysql://root:root@mysql:3306/user_data".into()
}

pub fn config() -> Cfg {
    (*self::CONFIG).clone()
}

lazy_static! {
    pub static ref CONFIG: Cfg = {
        dbg!(&envy::prefixed("").from_env::<Cfg>());
        match envy::prefixed("").from_env::<Cfg>() {
            Ok(val) => val,
            Err(e) => {
                panic!(format!("{}", e))
            }
        }
    };
}
