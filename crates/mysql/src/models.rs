mod hashes;
mod score_snaps;
mod scores;
mod songs;
mod user_statues;
mod users;

pub use hashes::*;
pub use score_snaps::*;
pub use scores::*;
pub use songs::*;
pub use user_statues::*;
pub use users::*;

pub(crate) type DieselResult<T> = Result<T, diesel::result::Error>;
