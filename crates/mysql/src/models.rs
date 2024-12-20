mod hashes;
mod player_stats;
mod score_snaps;
mod score_upload;
mod scores;
mod songs;
mod user_statues;
mod users;

pub use hashes::*;
pub use player_stats::*;
pub use score_snaps::*;
pub use score_upload::*;
pub use scores::*;
pub use songs::*;
pub use user_statues::*;
pub use users::*;

pub(crate) type DieselResult<T> = Result<T, diesel::result::Error>;
