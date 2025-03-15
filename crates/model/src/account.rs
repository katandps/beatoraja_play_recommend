mod score_upload;

pub use score_upload::{ScoreUpload, UploadAt, UploadId};

use parse_display::Display;
use serde::Serialize;
use {chrono::NaiveDateTime, serde::Deserialize};

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

#[derive(Clone, Debug, Serialize)]
pub struct RegisteredDate(NaiveDateTime);

impl RegisteredDate {
    pub fn new(date: NaiveDateTime) -> Self {
        Self(date)
    }

    pub fn to_naive_date_time(&self) -> NaiveDateTime {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct UserId(i32);

impl UserId {
    pub fn new(id: i32) -> Self {
        UserId(id)
    }

    pub fn get(&self) -> i32 {
        self.0
    }
}

#[derive(Clone, Debug, Serialize, Display)]
pub struct GoogleId(String);

impl GoogleId {
    pub fn new(id: String) -> Self {
        GoogleId(id)
    }
}

#[derive(Clone, Debug, Serialize, Display)]
pub struct GmailAddress(String);

impl GmailAddress {
    pub fn new(email: String) -> Self {
        GmailAddress(email)
    }
}

#[derive(Clone, Debug, Serialize, Display)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> Self {
        UserName(name)
    }
}

#[derive(Clone, Debug, Serialize, Display)]
pub struct Visibility(bool);

impl Visibility {
    pub fn new(v: bool) -> Visibility {
        Visibility(v)
    }

    pub fn to_bool(&self) -> bool {
        self.0
    }
}

#[derive(Deserialize)]
pub struct ChangeNameQuery {
    pub changed_name: String,
}

#[derive(Deserialize)]
pub struct ChangeVisibilityQuery {
    pub visibility: bool,
}
