use crate::config;
use crate::out::Out;
use send_slack::send_async;
use std::str::FromStr;
use std::string::ParseError;

pub enum Output {
    JSON,
    TEXT,
    STDOUT,
    SLACK,
}
impl Output {
    pub fn convert(&self, initial: Out) -> Out {
        match initial {
            Out::Result(r) => match self {
                Self::JSON => match serde_json::to_string(&r) {
                    Ok(j) => Out::Json(j),
                    Err(_) => Out::None,
                },
                Self::TEXT => Out::Text(r.to_string()),
                Self::STDOUT => {
                    println!("{}", r.to_string());
                    Out::None
                }
                Self::SLACK => {
                    println!("Can not send to slack");
                    Out::None
                }
            },
            _ => Out::None,
        }
    }

    pub async fn convert_async(&self, initial: Out) -> Out {
        match initial {
            Out::Result(r) => match self {
                Self::JSON => match serde_json::to_string(&r) {
                    Ok(j) => Out::Json(j),
                    Err(_) => Out::None,
                },
                Self::TEXT => Out::Text(r.to_string()),
                Self::STDOUT => {
                    println!("{}", r.to_string());
                    Out::None
                }
                Self::SLACK => {
                    let config = config();
                    let result = send_async(
                        config.slack_channel(),
                        config.slack_file_name(),
                        format!("{}", r.to_string()),
                    )
                    .await;
                    match result {
                        Ok(_) => Out::None,
                        Err(e) => panic!(e),
                    }
                }
            },
            _ => Out::None,
        }
    }
}

impl FromStr for Output {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "STDOUT" => Ok(Output::STDOUT),
            "JSON" => Ok(Output::JSON),
            "TEXT" => Ok(Output::TEXT),
            "SLACK" => Ok(Output::SLACK),
            _ => Ok(Output::STDOUT),
        }
    }
}
