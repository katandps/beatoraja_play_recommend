use crate::models::DieselResult;
use crate::schema::*;
use crate::MySqlPooledConnection;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = hashes)]
pub struct Hash {
    pub sha256: String,
    pub md5: String,
}

impl Hash {
    pub fn all(connection: &mut MySqlPooledConnection) -> DieselResult<Vec<Self>> {
        use crate::schema::hashes::dsl::*;
        hashes.load(connection)
    }

    pub fn for_tables(
        md5list: &[&str],
        connection: &mut MySqlPooledConnection,
    ) -> DieselResult<Vec<Self>> {
        use crate::schema::hashes::dsl::*;
        hashes.filter(md5.eq_any(md5list)).load(connection)
    }
}
