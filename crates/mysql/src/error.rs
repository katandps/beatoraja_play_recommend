use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DieselError: {0:?}")]
    DieselError(diesel::result::Error),
}
