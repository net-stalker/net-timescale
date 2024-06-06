use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("the data for type `{0}` is wrong, bad decode")]
    WrongDeletableData(String),
    #[error("Error while deleting `{0}`: `{1:?}`")]
    DbError(String, Box<dyn std::error::Error + Send + Sync>),
    #[error("Couldn't begin the transcation `0`")]
    TranscationErrorStart(String),
    #[error("Couldn't commit the transcation `0`")]
    TranscationErrorEnd(String),
    #[error("cound't delete file from the directory: {0}")]
    DeleteFile(String)
}
