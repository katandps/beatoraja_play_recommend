mod config;
mod error;
mod filter;
mod routes;
pub mod session;

use config::config;
use model::Songs;
use model::Tables;
use mysql::{MySQLClient, MySqlPool};
use repository::SongDataForTables;
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

    let mut client = MySQLClient::new(db_pool.get().unwrap());

    let tables = Arc::new(Mutex::new(Tables::default()));
    let song_data = Arc::new(Mutex::new(SongDB::default()));
    {
        let mut tables = tables.lock().await;
        let mut songs = song_data.lock().await;
        table::from_web(&mut tables).await;
        *songs = SongDB {
            song: client.song_data(&tables).await.unwrap(),
        }
    }
    let route = routes::routes(&db_pool, &tables, &song_data);

    let (http_addr, http_warp) = warp::serve(route.clone()).bind_ephemeral(([0, 0, 0, 0], 8000));
    let (https_addr, https_warp) = warp::serve(route)
        .tls()
        .cert_path(config().tls_cert_path.clone())
        .key_path(config().tls_key_path.clone())
        .bind_ephemeral(([0, 0, 0, 0], 4431));

    log::info!("Starting Listen with {:?} and {:?}", http_addr, https_addr);
    futures::future::join3(
        http_warp,
        https_warp,
        table_update(&tables, &song_data, &db_pool),
    )
    .await;
}

async fn table_update(tables: &TableData, song_data: &SongData, db_pool: &MySqlPool) {
    let mut client = MySQLClient::new(db_pool.get().unwrap());
    loop {
        tokio::time::sleep(Duration::from_secs(3600)).await;
        {
            log::info!("Starting to update difficulty tables.");
            let mut tables = tables.lock().await;
            table::from_web(&mut tables).await;
            let songs = client.song_data(&tables).await.unwrap();
            let song_db = Arc::clone(&song_data);
            song_db.lock().await.update(songs);
        }
    }
}

#[derive(Default)]
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
