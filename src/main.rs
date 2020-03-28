mod file;
mod db;
mod app;

pub mod schema;
pub mod model;
pub mod table;
pub mod whole_score;
pub mod song_data;
pub mod song;

#[macro_use]
extern crate diesel;

use crate::app::App;

fn main() {
    let app = App {
        table: file::get_table(),
        whole_score: db::score(),
        song_data: db::song_data(),
    };
    app.run()
}