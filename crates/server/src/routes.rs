mod custom_table;
mod uploads;

use crate::handler::*;
use crate::SongData;
use model::Tables;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub fn api_routes(
    db_pool: &MySqlPool,
    t: &Tables,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    health_route(db_pool)
        .or(account_route(db_pool))
        .or(users_route(db_pool))
        .or(change_name(db_pool))
        .or(change_visibility_route(db_pool))
        .or(logout())
        .or(tables_route(t))
        .or(songs_route(t, song_data))
        .or(detail_route(db_pool, t, song_data))
        .or(uploads::uploads(db_pool, song_data))
        .or(oauth_redirect_route(db_pool))
        .with(crate::cors_header())
        .with(warp::compression::gzip())
        .with(warp::log("api_access"))
        .boxed()
}

pub fn table_routes(
    db_pool: &MySqlPool,
    tables: &Tables,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    use custom_table::*;
    custom_table_header(tables)
        .or(custom_table_body(db_pool, tables, song_data))
        .or(custom_table())
        .with(crate::cors_header())
        .with(warp::log("table_access"))
        .boxed()
}
