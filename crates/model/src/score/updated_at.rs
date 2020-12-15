use crate::*;
use chrono::{DateTime, Duration, Local, TimeZone};
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
    fn day_start(self) -> UpdatedAt {
        Self::from_str(format!("{}", self.0.format("%Y-%m-%d")).as_str())
    }

    pub fn is_future(&self) -> bool {
        self > &UpdatedAt::day_start(UpdatedAt::now().sub(1))
    }

    pub fn sub(&self, days: i64) -> UpdatedAt {
        UpdatedAt(self.0 - Duration::days(days))
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
    }

    #[test]
    pub fn test_sub() {
        assert_eq!(
            "1992-11-20 00:00:00",
            UpdatedAt::from_str("1992-11-21").sub(1).to_string()
        )
    }

    #[test]
    pub fn test_future() {
        let date = UpdatedAt::day_start(UpdatedAt::now());
        assert_eq!(true, date.is_future());
        assert_eq!(false, date.sub(1).is_future());
    }
}
