use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::UpdatedAt;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SnapPeriod {
    #[serde(default)]
    pub since: DateTime<Utc>,
    #[serde(default = "until_default")]
    pub until: DateTime<Utc>,
}
fn until_default() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0).unwrap()
}

impl Default for SnapPeriod {
    fn default() -> Self {
        SnapPeriod {
            since: DateTime::default(),
            until: until_default(),
        }
    }
}

impl SnapPeriod {
    pub fn is_past_range(&self) -> bool {
        self.until < Utc::now()
    }

    pub fn contains(&self, date: &UpdatedAt) -> bool {
        date.is_contained(self)
    }

    pub fn from_until_timestamp(timestamp: i64) -> Self {
        SnapPeriod {
            since: DateTime::default(),
            until: Utc.timestamp_opt(timestamp, 0).unwrap(),
        }
    }
}
