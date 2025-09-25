use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::UpdatedAt;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SnapPeriod {
    pub since: DateTime<Utc>,
    pub until: DateTime<Utc>,
}

impl Default for SnapPeriod {
    fn default() -> Self {
        SnapPeriod {
            since: DateTime::default(),
            until: Utc.timestamp_opt(i64::MAX, 0).unwrap(),
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
