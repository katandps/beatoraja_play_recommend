use model::*;
use mysql::{MySQLClient, MySqlPool};
use std::convert::Infallible;
use warp::Filter;

pub fn with_db(
    db_pool: MySqlPool,
) -> impl Filter<Extract = (MySQLClient,), Error = Infallible> + Clone {
    warp::any().map(move || MySQLClient::new(db_pool.clone().get().unwrap()))
}

pub fn with_table(tables: Tables) -> impl Filter<Extract = (Tables,), Error = Infallible> + Clone {
    warp::any().map(move || tables.clone())
}
