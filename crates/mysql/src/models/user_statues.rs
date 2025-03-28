use crate::models::{DieselResult, User};
use crate::schema::*;
use crate::MySqlPooledConnection;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable)]
#[allow(unused)]
pub struct UserStatus {
    pub id: i32,
    pub user_id: i32,
    pub visible: bool,
    pub score_updated_at: NaiveDateTime,
}

impl UserStatus {
    pub fn visible_with_account(
        connection: &mut MySqlPooledConnection,
    ) -> DieselResult<Vec<(UserStatus, User)>> {
        use crate::schema::user_statuses::dsl::*;
        user_statuses
            .filter(visible.eq(true))
            .inner_join(users::table)
            .load(connection)
    }

    pub fn by_user(
        connection: &mut MySqlPooledConnection,
        user: &User,
    ) -> DieselResult<UserStatus> {
        use crate::schema::user_statuses::dsl::*;
        user_statuses.filter(user_id.eq(user.id)).first(connection)
    }

    pub fn delete_by_user(
        connection: &mut MySqlPooledConnection,
        user: &User,
    ) -> DieselResult<usize> {
        use crate::schema::user_statuses::dsl::*;
        diesel::delete(user_statuses.filter(user_id.eq(user.id))).execute(connection)
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = user_statuses)]
pub struct UserStatusForInsert {
    pub user_id: i32,
    pub visible: bool,
    pub score_updated_at: NaiveDateTime,
}
