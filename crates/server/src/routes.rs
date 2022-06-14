mod account;
mod change_name;
mod change_visibility;
mod custom_table;
mod detail;
mod health;
mod logout;
mod oauth_redirect;
mod ranking;
mod reset;
mod song_log;
mod songs;
mod stats;
mod tables;
mod upload;
mod users;

use upload::{play_data_upload_route, song_data_upload_route};

use crate::SongData;
use crate::TableData;
use mysql::MySqlPool;
use warp::filters::cors::Builder;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn routes(
    db_pool: &MySqlPool,
    tables: &TableData,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    api_routes(&db_pool, &tables, &song_data)
        .or(table_routes(&db_pool, &tables, &song_data))
        .recover(crate::error::handle_rejection)
        .boxed()
}

pub fn api_routes(
    db_pool: &MySqlPool,
    t: &TableData,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    health::route(db_pool)
        .or(account::route(db_pool))
        .or(users::route(db_pool))
        .or(change_name::route(db_pool))
        .or(change_visibility::route(db_pool))
        .or(logout::route())
        .or(tables::route(t))
        .or(stats::route(db_pool))
        .or(songs::route(t, song_data))
        .or(ranking::route(db_pool, song_data))
        .or(detail::route(db_pool, t, song_data))
        .or(play_data_upload_route(db_pool))
        .or(song_data_upload_route(db_pool, song_data))
        .or(reset::route(db_pool))
        .or(oauth_redirect::route(db_pool))
        .with(cors_header())
        .with(warp::compression::gzip())
        .with(warp::log("api_access"))
        .boxed()
}

pub fn table_routes(
    db_pool: &MySqlPool,
    tables: &TableData,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    custom_table::header_route(tables)
        .or(custom_table::body_route(db_pool, tables, song_data))
        .or(custom_table::table_route())
        .with(cors_header())
        .with(warp::log("table_access"))
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
            crate::session::SESSION_KEY,
        ])
}
