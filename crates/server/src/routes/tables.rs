use crate::filter::{with_table, with_tag};
use crate::map_response;
use table::TableClient;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn route(tables: &TableClient) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("tables"))
        .and(with_table(tables))
        .and(with_tag())
        .then(service::tables::get)
        .then(map_response)
        .boxed()
}
