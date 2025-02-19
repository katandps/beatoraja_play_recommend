use std::sync::Arc;

use model::TablesInfo;
use tokio::sync::{Mutex, OnceCell};

pub async fn tables() -> &'static Arc<Mutex<TablesInfo>> {
    static INSTANCE: OnceCell<Arc<Mutex<TablesInfo>>> = OnceCell::const_new();
    INSTANCE.get_or_init(init).await
}

async fn init() -> Arc<Mutex<TablesInfo>> {
    let tables = Arc::new(Mutex::new(TablesInfo::default()));
    {
        let mut m = tables.lock().await;
        table::from_with_cache(&mut m).await;
    }
    tables
}
