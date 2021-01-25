use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::Identifiable;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable)]
pub struct User {
    pub id: i32,
    pub google_id: String,
    pub gmail_address: String,
    pub name: String,
    pub registered_date: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "users"]
pub struct RegisteringUser {
    pub google_id: String,
    pub gmail_address: String,
    pub name: String,
    pub registered_date: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "rename_logs"]
pub struct RenameUser {
    pub user_id: i32,
    pub old_name: String,
    pub new_name: String,
    pub date: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable)]
pub struct UserStatus {
    pub id: i32,
    pub user_id: i32,
    pub visible: bool,
    pub score_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "user_statuses"]
pub struct UserStatusForInsert {
    pub user_id: i32,
    pub visible: bool,
    pub score_updated_at: NaiveDateTime,
}
