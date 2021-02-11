use crate::models::DieselResult;
use crate::schema::*;
use crate::MySqlPooledConnection;

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "hashes"]
pub struct Hash {
    pub sha256: String,
    pub md5: String,
}

impl Hash {
    pub fn all(connection: &MySqlPooledConnection) -> DieselResult<Vec<Self>> {
        use crate::schema::hashes::dsl::*;
        hashes.load(connection)
    }
}
