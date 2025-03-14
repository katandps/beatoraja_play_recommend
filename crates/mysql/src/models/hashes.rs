use crate::models::DieselResult;
use crate::schema::*;
use crate::MySqlPooledConnection;
use diesel::prelude::*;
use futures::lock::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::OnceLock;

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

    pub async fn for_tables(
        md5list: &[&str],
        connection: &mut MySqlPooledConnection,
    ) -> DieselResult<Vec<Self>> {
        use crate::schema::hashes::dsl::*;
        let cached = for_tables_is_cached()
            .try_lock()
            .map(|cached| cached.load(Ordering::Relaxed))
            .unwrap_or_default();
        if cached {
            Ok(for_tables_cache().lock().await.clone())
        } else {
            let mut cache = for_tables_cache().lock().await;
            let result = hashes.filter(md5.eq_any(md5list)).load(connection).unwrap();
            *cache = result.clone();
            let mut cached = for_tables_is_cached().try_lock().unwrap();
            *cached = AtomicBool::new(true);
            Ok(result)
        }
    }

    pub fn insert_new_hashes(
        new_hashes: Vec<Self>,
        connection: &mut MySqlPooledConnection,
    ) -> DieselResult<()> {
        use crate::schema;
        if new_hashes.is_empty() {
            return Ok(());
        }
        {
            let mut cached = for_tables_is_cached().try_lock().unwrap();
            *cached = AtomicBool::new(false);
        }
        for records in new_hashes.chunks(1000) {
            log::info!("Insert {} hashes.", records.len());
            diesel::insert_into(schema::hashes::table)
                .values(records)
                .execute(connection)?;
        }
        Ok(())
    }
}
fn for_tables_is_cached() -> &'static Arc<Mutex<AtomicBool>> {
    static INSTANCE: OnceLock<Arc<Mutex<AtomicBool>>> = OnceLock::new();
    INSTANCE.get_or_init(Arc::default)
}
fn for_tables_cache() -> &'static Arc<Mutex<Vec<Hash>>> {
    static INSTANCE: OnceLock<Arc<Mutex<Vec<Hash>>>> = OnceLock::new();
    INSTANCE.get_or_init(Arc::default)
}
