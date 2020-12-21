pub mod gmail_address;
pub mod registered_date;
pub mod user_name;

use crate::account::gmail_address::GmailAddress;
use crate::account::registered_date::RegisteredDate;
use crate::account::user_name::UserName;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Account {
    pub gmail_address: GmailAddress,
    pub name: UserName,
    pub registered_date: RegisteredDate,
}

impl Account {
    pub fn new(
        gmail_address: GmailAddress,
        name: UserName,
        registered_date: RegisteredDate,
    ) -> Self {
        Self {
            gmail_address,
            name,
            registered_date,
        }
    }

    pub fn email(&self) -> String {
        self.gmail_address.to_string()
    }

    pub fn user_name(&self) -> String {
        self.name.to_string()
    }

    pub fn registered(&self) -> NaiveDateTime {
        self.registered_date.to_naive_date_time()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = UserName::new(new_name);
    }
}
