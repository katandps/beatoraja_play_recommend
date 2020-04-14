pub fn send(channel: String, title: String, content: String) -> anyhow::Result<String> {
    let res = reqwest::blocking::Client::new()
        .get("https://slack.com/api/files.upload")
        .query(&[
            ("token", config::config().slack_bot_token()),
            ("title", title),
            ("channels", channel),
            ("pretty", "1".into()),
            ("content", content),
        ])
        .send()?;
    Ok(format!("{:#?}", res))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println! {"{}",send("テスト用".into(),"タイトル".into(), "内容".into()).unwrap()};
    }
}
