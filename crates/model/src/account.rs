use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Account {
    gmail_address: String,
    name: String,
    registered_date: NaiveDateTime,
}

impl Account {
    pub fn new(gmail_address: String, name: String, registered_date: NaiveDateTime) -> Self {
        Self {
            gmail_address,
            name,
            registered_date,
        }
    }
}
