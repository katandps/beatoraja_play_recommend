mod gmail_address;
mod google_id;
mod prelude;
mod registered_date;
mod user_id;
mod user_name;
mod visibility;

pub use prelude::*;

use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Account {
    pub user_id: UserId,
    pub google_id: GoogleId,
    pub gmail_address: GmailAddress,
    pub name: UserName,
    pub registered_date: RegisteredDate,
    pub visibility: Visibility,
}

impl Account {
    pub fn new(
        user_id: UserId,
        google_id: GoogleId,
        gmail_address: GmailAddress,
        name: UserName,
        registered_date: RegisteredDate,
        visibility: Visibility,
    ) -> Self {
        Self {
            user_id,
            google_id,
            gmail_address,
            name,
            registered_date,
            visibility,
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn google_id(&self) -> String {
        self.google_id.to_string()
    }

    pub fn email(&self) -> String {
        self.gmail_address.to_string()
    }

    pub fn user_name(&self) -> String {
        self.name.to_string()
    }

    pub fn visibility(&self) -> bool {
        self.visibility.to_bool()
    }

    pub fn registered(&self) -> NaiveDateTime {
        self.registered_date.to_naive_date_time()
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = UserName::new(new_name.to_string());
    }

    pub fn set_visibility(&mut self, new_visibility: bool) {
        self.visibility = Visibility::new(new_visibility)
    }
}
