pub mod cache_tags;
mod config;
mod error;
mod filter;
mod routes;
pub mod session;

use config::config;
use model::TablesInfo;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub type TableData = Arc<Mutex<TablesInfo>>;

#[tokio::main]
async fn main() {
    env_logger::init();
    let db_pool = mysql::get_db_pool();
    let tables = Arc::new(Mutex::new(TablesInfo::default()));
    {
        let mut tables = tables.lock().await;
        table::from_with_cache(&mut tables).await;
    }
    let route = routes::routes(&db_pool, &tables);

    let (http_addr, http_warp) = warp::serve(route.clone()).bind_ephemeral(([0, 0, 0, 0], 8000));
    let (https_addr, https_warp) = warp::serve(route)
        .tls()
        .cert_path(config().tls_cert_path.clone())
        .key_path(config().tls_key_path.clone())
        .bind_ephemeral(([0, 0, 0, 0], 4431));

    log::info!("Starting Listen with {:?} and {:?}", http_addr, https_addr);
    futures::future::join3(http_warp, https_warp, table_update(&tables)).await;
}

async fn table_update(tables: &TableData) {
    loop {
        tokio::time::sleep(Duration::from_secs(3600)).await;
        {
            log::info!("Starting to update difficulty tables.");
            let mut tables = tables.lock().await;
            table::from_web(&mut tables).await;
        }
    }
}
