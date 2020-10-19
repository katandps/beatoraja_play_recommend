use std::str::FromStr;
use std::string::ParseError;

pub enum Output {
    JSON,
    TEXT,
    STDOUT,
    SLACK,
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
