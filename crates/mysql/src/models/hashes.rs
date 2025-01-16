use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::RwLock;

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
        let cached = for_tables_refreshed()
            .try_lock()
            .map(|cached| cached.load(Ordering::Relaxed))
            .unwrap_or_default();
        if cached {
            Ok(for_tables_cache().try_read().unwrap().clone())
        } else {
            let mut cache = for_tables_cache().try_write().unwrap();
            let result = hashes.filter(md5.eq_any(md5list)).load(connection)?;
            *cache = result.clone();
            let mut cached = for_tables_refreshed().try_lock().unwrap();
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
        let mut cached = for_tables_refreshed().try_lock().unwrap();
        *cached = AtomicBool::new(true);
        let mut index = 0;
        loop {
            let mut records = Vec::new();
            while index < new_hashes.len() && records.len() < 1000 {
                records.push(new_hashes[index].clone());
                index += 1;
            }
            if records.is_empty() {
                break;
            }
            log::info!("Insert {} hashes.", records.len());
            diesel::insert_into(schema::hashes::table)
                .values(records)
                .execute(connection)?;
        }
        Ok(())
    }
}
fn for_tables_refreshed() -> &'static Arc<Mutex<AtomicBool>> {
    static INSTANCE: OnceLock<Arc<Mutex<AtomicBool>>> = OnceLock::new();
    INSTANCE.get_or_init(Arc::default)
}
fn for_tables_cache() -> &'static Arc<RwLock<Vec<Hash>>> {
    static INSTANCE: OnceLock<Arc<RwLock<Vec<Hash>>>> = OnceLock::new();
    INSTANCE.get_or_init(Arc::default)
}
