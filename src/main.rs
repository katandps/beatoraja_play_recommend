mod app;
mod db;
mod file;

pub mod model;
pub mod schema;
pub mod score;
pub mod scored_table;
pub mod song;
pub mod song_data;
pub mod table;

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
