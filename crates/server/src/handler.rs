mod account;
mod change_name;
mod change_visibility;
mod custom_table;
mod detail;
mod health;
mod logout;
mod oauth_redirect;
mod songs;
mod tables;
mod upload;
mod users;

pub use account::account_route;
pub use change_name::change_name;
pub use change_visibility::change_visibility_route;
pub use custom_table::*;
pub use detail::detail_route;
pub use health::health_route;
pub use logout::logout;
pub use oauth_redirect::oauth_redirect_route;
pub use songs::songs_route;
pub use tables::tables_route;
pub use upload::{score_log_upload_route, score_upload_route, song_data_upload_route};
pub use users::users_route;
