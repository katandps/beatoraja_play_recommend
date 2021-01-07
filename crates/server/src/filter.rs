use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use model::*;
use r2d2::Pool;
use std::convert::Infallible;
use warp::Filter;

pub fn with_db(
    db_pool: Pool<ConnectionManager<MysqlConnection>>,
) -> impl Filter<Extract = (Pool<ConnectionManager<MysqlConnection>>,), Error = Infallible> + Clone
{
    warp::any().map(move || db_pool.clone())
}

pub fn with_table(tables: Tables) -> impl Filter<Extract = (Tables,), Error = Infallible> + Clone {
    warp::any().map(move || tables.clone())
}
