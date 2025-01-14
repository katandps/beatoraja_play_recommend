use crate::{schema::*, MySqlPooledConnection};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use model::{SessionKey, UserId};

use super::DieselResult;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = revoked_sessions)]
pub struct SessionRevoke {
    pub session_key: String,
    pub user_id: i32,
    pub revoked_at: NaiveDateTime,
}

impl SessionRevoke {
    pub fn new(session_key: &SessionKey, user_id: UserId) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            session_key: session_key.to_string(),
            user_id: user_id.get(),
            revoked_at: now,
        }
    }
}

#[derive(Debug, Clone, Queryable)]
#[diesel(table_name = revoked_sessions)]
#[allow(dead_code)]
pub struct RevokedSession {
    pub id: i32,
    pub session_key: String,
    pub user_id: i32,
    pub revoked_at: NaiveDateTime,
}

impl RevokedSession {
    pub fn revoked(
        connection: &mut MySqlPooledConnection,
        session: &str,
        query_user_id: i32,
    ) -> DieselResult<Vec<Self>> {
        use crate::schema::revoked_sessions::dsl::*;
        revoked_sessions
            .filter(user_id.eq(query_user_id))
            .filter(session_key.eq(session))
            .load(connection)
    }
}
