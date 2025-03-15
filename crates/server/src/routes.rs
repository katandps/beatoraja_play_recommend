mod authorization;
mod custom_table;
mod general;
mod logged_in;
use futures::lock::Mutex;
use mysql::MySqlPool;
use service::songs::SongsTag;
use std::sync::Arc;
use table::TableClient;
use warp::filters::cors::Builder;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn routes(db_pool: &MySqlPool, tables: TableClient) -> BoxedFilter<(impl Reply,)> {
    let songs_tag = Arc::new(Mutex::new(SongsTag::new()));

    general::routes(db_pool, tables.clone(), &songs_tag)
        .or(logged_in::routes(db_pool, &songs_tag))
        .or(authorization::routes(db_pool))
        .or(custom_table::routes(db_pool, tables))
        .with(cors_header())
        .with(warp::log("api_access"))
        .boxed()
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
            crate::SESSION_KEY,
        ])
}
