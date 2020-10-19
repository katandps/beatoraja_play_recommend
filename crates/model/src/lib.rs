mod app;
mod command;
mod prelude;
mod score;
mod scored_table;
mod song;
mod summary;
mod table;

pub use prelude::*;

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
