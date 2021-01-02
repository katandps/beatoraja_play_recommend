use crate::*;
use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone};
use std::fmt;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct UpdatedAt(DateTime<Local>);

impl UpdatedAt {
    pub fn new() -> UpdatedAt {
        UpdatedAt(DateTime::from(Local.timestamp(0, 0)))
    }
    pub fn from_timestamp(timestamp: i64) -> UpdatedAt {
        UpdatedAt(DateTime::from(Local.timestamp(timestamp, 0)))
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

    pub fn naive_datetime(&self) -> NaiveDateTime {
        self.0.naive_local()
    }

    pub fn from_naive_datetime(time: NaiveDateTime) -> Self {
        Self::from_timestamp((time - Duration::hours(9)).timestamp())
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

    #[test]
    pub fn test_cmp() {
        let date1 = UpdatedAt::now();
        let date2 = date1.sub(1);

        assert!(date1 > date2);
    }

    #[test]
    pub fn test_naive_datetime() {
        let date1 = UpdatedAt::from_timestamp(1000000000);
        let date2 = UpdatedAt::from_naive_datetime(date1.naive_datetime());
        assert_eq!(date1, date2);
    }
}
