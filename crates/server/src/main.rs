mod config;
mod filter;
mod routes;

use config::config;
use serde::Serialize;
use std::time::Duration;
use table::TableClient;
use warp::http;
use warp::reject::Rejection;
use warp::reply::Reply;

#[tokio::main]
async fn main() {
    match std::env::var("RUST_JSON_LOG") {
        Ok(str) if &str != "0" => tracing_subscriber::fmt::init(),
        _ => tracing_subscriber::fmt().json().init(),
    }
    let db_pool = mysql::get_db_pool();
    let tables = TableClient::new();
    let _ = tables.init().await;

    let route = routes::routes(&db_pool, tables.clone());

    let (http_addr, http_warp) = warp::serve(route.clone()).bind_ephemeral(([0, 0, 0, 0], 8000));
    let (https_addr, https_warp) = warp::serve(route)
        .tls()
        .cert_path(config().tls_cert_path.clone())
        .key_path(config().tls_key_path.clone())
        .bind_ephemeral(([0, 0, 0, 0], 4431));

    log::info!("Starting Listen with {:?} and {:?}", http_addr, https_addr);
    futures::future::join3(http_warp, https_warp, table_update(&tables)).await;
}

async fn table_update(tables: &TableClient) {
    loop {
        tokio::time::sleep(Duration::from_secs(3600)).await;
        let result = tables.update().await;
        if result.is_err() {
            log::warn!("{:?}", result)
        }
    }
}

pub async fn json<T: Serialize>(result: anyhow::Result<service::Response<T>>) -> impl Reply {
    match result {
        Ok(service::Response::Ok { tag, body }) => {
            let mut builder =
                http::Response::builder().header("Content-type", "application/json; charset=utf-8");

            if let Some(tag) = tag {
                builder = builder.header("ETag", tag);
            }
            let json = serde_json::to_string(&body).unwrap();
            builder.body(json).unwrap()
        }
        Ok(service::Response::Cached { tag }) => http::Response::builder()
            .status(http::StatusCode::NOT_MODIFIED)
            .header("Content-type", "application/json; charset=utf-8")
            .header("ETag", tag)
            .body("".to_string())
            .unwrap(),
        Err(e) => {
            log::error!("{:?}", e);
            http::Response::builder()
                .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body("Internal server error".to_string())
                .unwrap()
        }
    }
}

pub async fn query<T>(query: anyhow::Result<T>) -> Result<T, Rejection> {
    Ok(query.unwrap())
}
pub const SESSION_KEY: &str = "session-token";
