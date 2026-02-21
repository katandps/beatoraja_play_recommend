use model::Account;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct AccountResponse {
    pub user_id: i32,
    pub google_id: String,
    pub gmail_address: String,
    pub name: String,
    pub registered_date: String,
    pub visibility: bool,
}

impl AccountResponse {
    pub fn from_account(account: &Account) -> Self {
        Self {
            user_id: account.user_id.get(),
            google_id: account.google_id.to_string(),
            gmail_address: account.gmail_address.to_string(),
            name: account.name.to_string(),
            registered_date: account.registered_date.to_naive_date_time().to_string(),
            visibility: account.visibility.to_bool(),
        }
    }
}
