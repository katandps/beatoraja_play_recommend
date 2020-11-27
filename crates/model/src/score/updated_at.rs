use crate::*;
use chrono::{DateTime, Local, TimeZone};
use std::fmt;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct UpdatedAt(DateTime<Local>);

impl UpdatedAt {
    pub fn new() -> UpdatedAt {
        UpdatedAt(DateTime::from(Local.timestamp(0, 0)))
    }
    pub fn from_timestamp(timestamp: i32) -> UpdatedAt {
        UpdatedAt(DateTime::from(Local.timestamp(timestamp as i64, 0)))
    }
    pub fn from_str(str: &str) -> UpdatedAt {
        match DateTime::parse_from_rfc3339(format!("{}T00:00:00+09:00", str).as_str()) {
            Ok(d) => UpdatedAt(DateTime::from(d)),
            _ => Self::now(),
        }
    }
    pub fn now() -> UpdatedAt {
        UpdatedAt(Local::now())
    }

    pub fn is_today(&self) -> bool {
        self.0 == Local::now()
    }
}

impl fmt::Display for UpdatedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M:%S"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            "1992-11-20 00:00:00",
            UpdatedAt::from_str("1992-11-20").to_string()
        );
        assert_eq!(
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            UpdatedAt::from_str("hogehoge").to_string()
        );
    }
}
