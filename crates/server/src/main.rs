mod config;
mod error;
mod filter;
mod routes;
pub mod session;

use config::config;
use model::Songs;
use mysql::MySQLClient;
use warp::Filter;

#[macro_use]
extern crate lazy_static;

use model::Tables;
use repository::AllSongData;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub type SongData = Arc<Mutex<SongDB>>;
pub type TableData = Arc<Mutex<Tables>>;

#[tokio::main]
async fn main() {
    env_logger::init();
    let db_pool = mysql::get_db_pool();

    let client = MySQLClient::new(db_pool.get().unwrap());
    let song_data = Arc::new(Mutex::new(SongDB {
        song: client.song_data().unwrap(),
    }));
    let tables = Arc::new(Mutex::new(Tables::default()));
    {
        let mut tables = tables.lock().await;
        table::from_web(&mut tables).await;
    }
    let table_route = routes::table_routes(&db_pool, &tables, &song_data);
    let route = routes::api_routes(&db_pool, &tables, &song_data)
        .or(table_route)
        .recover(error::handle_rejection);

    let (http_addr, http_warp) = warp::serve(route.clone()).bind_ephemeral(([0, 0, 0, 0], 8000));
    let (https_addr, https_warp) = warp::serve(route.clone())
        .tls()
        .cert_path(config().tls_cert_path)
        .key_path(config().tls_key_path)
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

pub struct SongDB {
    song: Songs,
}

impl SongDB {
    pub fn update(&mut self, new: Songs) {
        self.song = new;
    }
}

impl Deref for SongDB {
    type Target = Songs;

    fn deref(&self) -> &Self::Target {
        &self.song
    }
}
