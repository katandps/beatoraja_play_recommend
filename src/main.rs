mod file;
mod db;

pub mod schema;
pub mod model;

#[macro_use]
extern crate diesel;

pub use diesel::prelude::*;

fn main() {
    file::run();
    db::run();
}