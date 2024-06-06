use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("the data for type `{0}` is wrong, bad decode")]
    WrongUpdatableData(String),
    #[error("Error while updating `{0}`: `{1:?}`")]
    DbError(String, Box<dyn std::error::Error + Send + Sync>),
    #[error("Transcation error: `0`")]
    TranscationError(String),
    #[error("Something went wrong during pcaps update `0`")]
    CouldntUpdatePcaps(String),
}
