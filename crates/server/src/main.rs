mod error;
mod filter;
mod handler;
mod session;

use config::config;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use r2d2::Pool;
use std::collections::HashMap;
use std::env;
use table::get_tables;
use warp::Filter;

const TLS_CERT_PATH: &str = "TLS_CERT_PATH";
const TLS_CERT_PATH_DEFAULT: &str = "./files/cert.pem";
const TLS_KEY_PATH: &str = "TLS_KEY_PATH";
const TLS_KEY_PATH_DEFAULT: &str = "./files/key.rsa";

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let log = warp::log("example");

    let db_pool = get_db_pool();

    let tables = get_tables(false).await;
    let tables_route = warp::get()
        .and(warp::path("tables"))
        .and(filter::with_table(tables.clone()))
        .and(warp::path::end())
        .and_then(handler::table_handler);

    let health_route = warp::get()
        .and(warp::path("health"))
        .and(filter::with_db(db_pool.clone()))
        .and_then(handler::health::health_handler);

    let account_route = warp::get()
        .and(warp::path("account"))
        .and(warp::header::<String>(session::SESSION_KEY))
        .and_then(handler::account_handler);

    let change_name_route = warp::post()
        .and(warp::path("update"))
        .and(warp::path("name"))
        .and(warp::header::<String>(session::SESSION_KEY))
        .and(warp::body::json())
        .and_then(handler::change_name_handler);

    let logout_route = warp::get()
        .and(warp::path("logout"))
        .and(warp::header::<String>(session::SESSION_KEY))
        .and_then(handler::logout_handler);

    let my_detail_route = warp::get()
        .and(warp::path("my_detail"))
        .and(warp::path::end())
        .and(filter::with_table(tables.clone()))
        .and(warp::header::<String>(session::SESSION_KEY))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handler::detail::my_detail_handler);

    let detail_route = warp::get()
        .and(warp::path("detail"))
        .and(warp::path::end())
        .and(filter::with_table(tables.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handler::detail::detail_handler);

    let score_upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::path("score"))
        .and(warp::multipart::form().max_length(100 * 1024 * 1024))
        .and(warp::header::<String>(session::SESSION_KEY))
        .and_then(handler::upload::upload_score_handler);

    let scorelog_upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::path("score_log"))
        .and(warp::multipart::form().max_length(100 * 1024 * 1024))
        .and(warp::header::<String>(session::SESSION_KEY))
        .and_then(handler::upload::upload_score_log_handler);

    let songdata_upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::path("song_data"))
        .and(warp::multipart::form().max_length(100 * 1024 * 1024))
        .and(warp::header::<String>(session::SESSION_KEY))
        .and_then(handler::upload::upload_song_data_handler);

    let oauth_redirect_route = warp::get()
        .and(warp::path("oauth"))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handler::oauth);

    let route = health_route
        .or(account_route)
        .or(change_name_route)
        .or(logout_route)
        .or(tables_route)
        .or(detail_route)
        .or(my_detail_route)
        .or(score_upload_route)
        .or(scorelog_upload_route)
        .or(songdata_upload_route)
        .or(oauth_redirect_route)
        .recover(error::handle_rejection)
        .with(
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
                ]),
        )
        .with(log);

    let (_http_addr, http_warp) = warp::serve(route.clone()).bind_ephemeral(([0, 0, 0, 0], 8000));

    let (_https_addr, https_warp) = warp::serve(route.clone())
        .tls()
        .cert_path(get_env(TLS_CERT_PATH, TLS_CERT_PATH_DEFAULT))
        .key_path(get_env(TLS_KEY_PATH, TLS_KEY_PATH_DEFAULT))
        .bind_ephemeral(([0, 0, 0, 0], 4431));

    futures::future::join(http_warp, https_warp).await;
}

fn get_env(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => default.into(),
    }
}

fn get_db_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    Pool::builder().build_unchecked(ConnectionManager::new(config().mysql_url))
}
