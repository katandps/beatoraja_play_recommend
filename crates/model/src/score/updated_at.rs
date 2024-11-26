use crate::*;
use anyhow::Result;
use chrono::{DateTime, Duration, NaiveDateTime, ParseError, TimeZone, Utc};
use std::fmt;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize, Hash)]
pub struct UpdatedAt(DateTime<Utc>);

impl Default for UpdatedAt {
    fn default() -> UpdatedAt {
        UpdatedAt(Utc.timestamp_opt(0, 0).unwrap())
    }
}

impl UpdatedAt {
    pub fn from_timestamp(timestamp: i64) -> UpdatedAt {
        UpdatedAt(Utc.timestamp_opt(timestamp, 0).unwrap())
    }

    pub fn now() -> UpdatedAt {
        UpdatedAt(Utc::now())
    }
    fn day_start(self) -> UpdatedAt {
        Self::from_str(format!("{}", self.0.format("%Y-%m-%d")).as_str()).expect("bugged")
    }

    pub fn is_future(&self) -> bool {
        self > &UpdatedAt::day_start(&UpdatedAt::now() - Duration::days(1))
    }

    pub fn naive_datetime(&self) -> NaiveDateTime {
        self.0.naive_utc()
    }

    pub fn from_naive_datetime(time: NaiveDateTime) -> Self {
        Self::from_timestamp(time.and_utc().timestamp())
    }
}

impl FromStr for UpdatedAt {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = DateTime::parse_from_rfc3339(format!("{}T00:00:00+00:00", s).as_str())?;
        Ok(UpdatedAt(DateTime::from(date)))
    }
}

///
/// 日付を巻き戻す意味合いになる
///
impl Sub<Duration> for &UpdatedAt {
    type Output = UpdatedAt;

    fn sub(self, rhs: Duration) -> Self::Output {
        UpdatedAt(self.0 - rhs)
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
            UpdatedAt::from_str("1992-11-20").unwrap().to_string()
        );
    }

    #[test]
    pub fn test_sub() {
        assert_eq!(
            "1992-11-20 00:00:00",
            (&UpdatedAt::from_str("1992-11-21").unwrap() - Duration::days(1)).to_string()
        )
    }

    #[test]
    pub fn test_future() {
        let date = UpdatedAt::day_start(UpdatedAt::now());
        assert_eq!(true, date.is_future());
        assert_eq!(false, (&date - Duration::days(1)).is_future());
    }

    #[test]
    pub fn test_cmp() {
        let date1 = UpdatedAt::now();
        let date2 = &date1 - Duration::days(1);

        assert!(date1 > date2);
    }

    #[test]
    pub fn test_naive_datetime() {
        let date1 = UpdatedAt::from_timestamp(1000000000);
        let date2 = UpdatedAt::from_naive_datetime(date1.naive_datetime());
        assert_eq!(date1, date2);
    }
}
