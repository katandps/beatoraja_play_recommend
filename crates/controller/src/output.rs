use crate::config;
use crate::out::Out;
use send_slack::send;
use std::str::FromStr;
use std::string::ParseError;

pub enum Output {
    JSON,
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
                Self::STDOUT => {
                    println!("{}", r.to_string());
                    Out::None
                }
                Self::SLACK => {
                    let config = config();
                    let result = send(
                        config.slack_channel(),
                        config.slack_file_name(),
                        format!("{}", r.to_string()),
                    );
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
            "SLACK" => Ok(Output::SLACK),
            _ => Ok(Output::STDOUT),
        }
    }
}
