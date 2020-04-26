use reqwest::blocking::multipart::Form;
use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;
use std::io::Write;

pub fn send(channel: String, title: String, content: String) -> anyhow::Result<String> {
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
}
