use crate::config;
use crate::out::Out;
use send_slack::send;

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
                    let _ = send(
                        config.slack_channel(),
                        config.slack_file_name(),
                        format!("{}", r.to_string()),
                    );
                    Out::None
                }
            },
            _ => Out::None,
        }
    }
}
