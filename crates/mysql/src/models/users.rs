use crate::models::DieselResult;
use crate::schema::*;
use crate::MySqlPooledConnection;
use chrono::{NaiveDateTime, Utc};
use diesel::Identifiable;
use model::*;
use oauth_google::GoogleProfile;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable)]
pub struct User {
    pub id: i32,
    pub google_id: String,
    pub gmail_address: String,
    pub name: String,
    pub registered_date: NaiveDateTime,
}

impl From<User> for Account {
    fn from(user: User) -> Self {
        Account::new(
            UserId::new(user.id),
            GoogleId::new(user.google_id),
            GmailAddress::new(user.gmail_address),
            UserName::new(user.name),
            RegisteredDate::new(user.registered_date),
            Visibility::new(false),
        )
    }
}

impl User {
    pub fn by_account(connection: &MySqlPooledConnection, account: &Account) -> DieselResult<Self> {
        use crate::schema::users::dsl::*;
        users
            .filter(gmail_address.eq(account.email()))
            .first(connection)
    }

    pub fn by_google_profile(
        connection: &MySqlPooledConnection,
        profile: &GoogleProfile,
    ) -> DieselResult<Self> {
        use crate::schema::users::dsl::*;
        users
            .filter(gmail_address.eq(&profile.email))
            .first(connection)
    }

    pub fn by_google_id(
        connection: &MySqlPooledConnection,
        google_id_string: String,
    ) -> DieselResult<Self> {
        use crate::schema::users::dsl::*;
        users
            .filter(google_id.eq(google_id_string))
            .first(connection)
    }

    pub fn by_user_id(connection: &MySqlPooledConnection, user_id: i32) -> DieselResult<Self> {
        use crate::schema::users::dsl::*;
        users.filter(id.eq(user_id)).first(connection)
    }
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "users"]
pub struct RegisteringUser {
    pub google_id: String,
    pub gmail_address: String,
    pub name: String,
    pub registered_date: NaiveDateTime,
}

impl RegisteringUser {
    pub fn from_profile(profile: &GoogleProfile) -> RegisteringUser {
        RegisteringUser {
            google_id: profile.user_id.clone(),
            gmail_address: profile.email.clone(),
            name: profile.name.to_string(),
            registered_date: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "rename_logs"]
pub struct RenameUser {
    pub user_id: i32,
    pub old_name: String,
    pub new_name: String,
    pub date: NaiveDateTime,
}
