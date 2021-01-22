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
    let tables_route = tables(t);
    let songs_route = songs(t, song_data);
    let health_route = health(db_pool);
    let account_route = account(db_pool);
    let change_name_route = change_name(db_pool);
    let logout_route = logout();
    let detail_route = detail(db_pool, t, song_data);
    let upload_route = uploads::uploads(db_pool, song_data);
    let oauth_redirect_route = oauth_redirect_route(db_pool);
    health_route
        .or(account_route)
        .or(change_name_route)
        .or(logout_route)
        .or(tables_route)
        .or(songs_route)
        .or(detail_route)
        .or(upload_route)
        .or(oauth_redirect_route)
        .boxed()
}

pub fn table_routes(
    db_pool: &MySqlPool,
    t: &Tables,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    custom_table::custom_tables(db_pool, t, song_data)
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
        .and(path("update"))
        .and(path("name"))
        .and(with_db(&db_pool))
        .and(account_by_session(&db_pool))
        .and(changed_name_by_query())
        .and_then(change_name::change_name_handler)
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
