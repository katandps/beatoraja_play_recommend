mod config;
mod error;
mod filter;
mod handler;
mod routes;
mod session;

use config::config;
use warp::Filter;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let log = warp::log("example");
    let db_pool = mysql::get_db_pool();
    let new_tables = table::from_web().await;

    let route = routes::all_routes(&db_pool, &new_tables)
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

    let (http_addr, http_warp) = warp::serve(route.clone()).bind_ephemeral(([0, 0, 0, 0], 8000));
    let (https_addr, https_warp) = warp::serve(route.clone())
        .tls()
        .cert_path(config().tls_cert_path)
        .key_path(config().tls_key_path)
        .bind_ephemeral(([0, 0, 0, 0], 4431));

    dbg!(http_addr, https_addr);

    futures::future::join(http_warp, https_warp).await;
}
