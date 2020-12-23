use serde_json::Value;
use std::fs;
use std::io::Write;

pub fn send(channel: String, title: String, content: String) -> anyhow::Result<String> {
    use reqwest::blocking::multipart::Form;
    use reqwest::blocking::Client;

    let mut f = fs::File::create("buf.txt").unwrap();
    f.write_all(content.as_bytes()).unwrap();
    let form = Form::new().file("file", "buf.txt").unwrap();
    let res = Client::new()
        .post("https://slack.com/api/files.upload")
        .multipart(form)
        .query(&[
            ("token", config().slack_bot_token()),
            ("title", title),
            ("channels", channel),
            ("pretty", "1".into()),
        ])
        .send()?;
    let _ = fs::remove_file("buf.txt");
    match res.text() {
        Err(e) => Ok(String::from(format!("アップロード失敗:{:?}", e))),
        Ok(result) => {
            let v: Value = serde_json::from_str(result.as_str()).unwrap();

            match v["ok"].as_bool() {
                Some(true) => Ok(String::from(format!("アップロード完了"))),
                Some(false) => Ok(String::from(format!("アップロード失敗:{}", v["error"]))),
                None => Ok(String::from(format!("アップロード失敗:{}", v["error"]))),
            }
        }
    }
}

pub async fn send_async(content: String) -> anyhow::Result<String> {
    use reqwest::multipart::{Form, Part};
    use reqwest::Client;

    let (channel, title) = (config().slack_channel(), config().slack_file_name());
    let fp = Part::text(content).file_name("buf.txt");

    let form = Form::new().part("file", fp);
    let res = Client::new()
        .post("https://slack.com/api/files.upload")
        .multipart(form)
        .query(&[
            ("token", config().slack_bot_token()),
            ("title", title),
            ("channels", channel),
            ("pretty", "1".into()),
        ])
        .send()
        .await?
        .text()
        .await?;
    let v: Value = serde_json::from_str(res.as_str()).unwrap();

    match v["ok"].as_bool() {
        Some(true) => Ok(String::from(format!("アップロード完了"))),
        Some(false) => Ok(String::from(format!("アップロード失敗:{}", v["error"]))),
        None => Ok(String::from(format!("アップロード失敗:{}", v["error"]))),
    }
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
        println! {"{}",send("テスト用".into(),"タイトル".into(), "内容内容内容".into()).unwrap()};
    }

    #[tokio::test]
    async fn async_fn_works() {
        println!("{}", send_async("ほげほげ".into()).await.unwrap());
    }
}
