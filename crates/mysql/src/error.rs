use crate::error::Error::DieselError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DieselError: {0:?}")]
    DieselError(diesel::result::Error),
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Error {
        DieselError(e)
    }
}
