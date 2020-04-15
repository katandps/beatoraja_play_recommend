pub fn send(channel: String, title: String, content: String) -> anyhow::Result<String> {
    let res = reqwest::blocking::Client::new()
        .get("https://slack.com/api/files.upload")
        .query(&[
            ("token", config().slack_bot_token()),
            ("title", title),
            ("channels", channel),
            ("pretty", "1".into()),
            ("content", content),
        ])
        .send()?;
    Ok(format!("{:#?}", res))
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println! {"{}",send("テスト用".into(),"タイトル".into(), "内容".into()).unwrap()};
    }
}
