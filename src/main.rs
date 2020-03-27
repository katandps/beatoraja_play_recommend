mod file;
mod db;

pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;

pub use diesel::prelude::*;

fn main() {
    file::run();
    db::run();
}