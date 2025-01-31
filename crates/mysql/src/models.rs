mod hashes;
mod player_stats;
mod revoked_sessions;
mod score_snaps;
mod score_upload;
mod scores;
mod songs;
mod upload_stats;
mod user_statues;
mod users;

pub use hashes::*;
pub use player_stats::*;
pub use revoked_sessions::*;
pub use score_snaps::*;
pub use score_upload::*;
pub use scores::*;
pub use songs::*;
pub use upload_stats::*;
pub use user_statues::*;
pub use users::*;

pub(crate) type DieselResult<T> = Result<T, diesel::result::Error>;
