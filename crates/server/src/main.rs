mod config;
mod error;
mod filter;
mod handler;
mod routes;
mod session;

use config::config;
use model::Songs;
use mysql::MySQLClient;
use warp::Filter;

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use tokio::sync::Mutex;
use warp::filters::cors::Builder;

pub type SongData = Arc<Mutex<SongDB>>;

#[tokio::main]
async fn main() {
    env_logger::init();
    let log = warp::log("access");
    let db_pool = mysql::get_db_pool();
    let new_tables = table::from_web().await;

    let client = MySQLClient::new(db_pool.get().unwrap());
    let song_data = Arc::new(Mutex::new(SongDB {
        song: client.song_data().unwrap(),
    }));

    let table_route = routes::table_routes(&db_pool, &new_tables, &song_data)
        .with(cors_header())
        .with(log);
    let route = routes::api_routes(&db_pool, &new_tables, &song_data)
        .recover(error::handle_rejection)
        .with(cors_header())
        .with(warp::compression::gzip())
        .with(log)
        .or(table_route);

    let (http_addr, http_warp) = warp::serve(route.clone()).bind_ephemeral(([0, 0, 0, 0], 8000));
    let (https_addr, https_warp) = warp::serve(route.clone())
        .tls()
        .cert_path(config().tls_cert_path)
        .key_path(config().tls_key_path)
        .bind_ephemeral(([0, 0, 0, 0], 4431));

    log::info!("Starting Listen with {:?} and {:?}", http_addr, https_addr);
    futures::future::join(http_warp, https_warp).await;
}

fn cors_header() -> Builder {
    warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec![
            "x-requested-with",
            "origin",
            "referer",
            "x-csrftoken",
            "oauth-token",
            "content-type",
            "content-length",
            "accept",
            "accept-encoding",
            "accept-language",
            "user-agent",
            session::SESSION_KEY,
        ])
}

pub struct SongDB {
    song: Songs,
}

impl SongDB {
    pub fn update(&mut self, new: Songs) {
        self.song = new;
    }
}
