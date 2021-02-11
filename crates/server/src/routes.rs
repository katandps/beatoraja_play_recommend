mod custom_table;
mod uploads;

use crate::filter::*;
use crate::handler::*;
use crate::SongData;
use model::Tables;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn api_routes(
    db_pool: &MySqlPool,
    t: &Tables,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    health(db_pool)
        .or(account(db_pool))
        .or(users_route(db_pool))
        .or(change_name(db_pool))
        .or(change_visibility(db_pool))
        .or(logout())
        .or(tables(t))
        .or(songs(t, song_data))
        .or(detail(db_pool, t, song_data))
        .or(uploads::uploads(db_pool, song_data))
        .or(oauth_redirect_route(db_pool))
        .with(crate::cors_header())
        .with(warp::compression::gzip())
        .with(warp::log("api_access"))
        .boxed()
}

fn users_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("users"))
        .and(with_db(db_pool))
        .and_then(users::users_handler)
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

fn tables(tables: &Tables) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("tables"))
        .and(with_table(tables))
        .and_then(tables::table_handler)
        .boxed()
}
fn songs(tables: &Tables, songs: &SongData) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("songs"))
        .and(with_table(tables))
        .and(with_song_data(songs))
        .and_then(songs::songs_handler)
        .boxed()
}

fn health(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("health"))
        .and(with_db(&db_pool))
        .and_then(health::health_handler)
        .boxed()
}

fn account(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("account"))
        .and(account_by_session(&db_pool))
        .and_then(account::account_handler)
        .boxed()
}

fn change_name(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("user"))
        .and(path("name"))
        .and(with_db(db_pool))
        .and(account_by_session(db_pool))
        .and(changed_name_by_query())
        .and_then(change_name::change_name_handler)
        .boxed()
}

fn change_visibility(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("user"))
        .and(path("visibility"))
        .and(with_db(db_pool))
        .and(account_by_session(db_pool))
        .and(changed_visibility_by_query())
        .and_then(change_visibility::change_visibility_handler)
        .boxed()
}

fn logout() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("logout"))
        .and(receive_session_key())
        .and_then(logout::logout_handler)
        .boxed()
}

fn detail(
    db_pool: &MySqlPool,
    tables: &Tables,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("detail"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and(detail_query())
        .and(account_id_query(db_pool))
        .and(with_song_data(song_data))
        .and_then(detail::detail_handler)
        .boxed()
}

fn oauth_redirect_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("oauth"))
        .and(with_db(&db_pool))
        .and(google_oauth_code())
        .and_then(oauth_redirect::oauth_handler)
        .boxed()
}
