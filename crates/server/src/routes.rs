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
mod songs;
mod stats;
mod tables;
mod upload;
mod users;

use account::account_route;
use change_name::change_name;
use change_visibility::change_visibility_route;
use custom_table::*;
use detail::detail_route;
use health::health_route;
use logout::logout;
use oauth_redirect::oauth_redirect_route;
use ranking::ranking_route;
use reset::reset_route;
use songs::songs_route;
use stats::stats_route;
use tables::tables_route;
use upload::{play_data_upload_route, song_data_upload_route};
use users::users_route;

use crate::SongData;
use crate::TableData;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn api_routes(
    db_pool: &MySqlPool,
    t: &TableData,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    health_route(db_pool)
        .or(account_route(db_pool))
        .or(users_route(db_pool))
        .or(change_name(db_pool))
        .or(change_visibility_route(db_pool))
        .or(logout())
        .or(tables_route(t))
        .or(stats_route(db_pool))
        .or(songs_route(t, song_data))
        .or(ranking_route(db_pool, song_data))
        .or(detail_route(db_pool, t, song_data))
        .or(play_data_upload_route(db_pool))
        .or(song_data_upload_route(db_pool, song_data))
        .or(reset_route(db_pool))
        .or(oauth_redirect_route(db_pool))
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
    custom_table_header(tables)
        .or(custom_table_body(db_pool, tables, song_data))
        .or(custom_table_route())
        .with(cors_header())
        .with(warp::log("table_access"))
        .boxed()
}

use warp::filters::cors::Builder;
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
