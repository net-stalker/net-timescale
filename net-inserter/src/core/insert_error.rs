use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InsertError {
    #[error("the data for type `{0}` is wrong, bad decode")]
    WrongInsertableData(String),
    #[error("Error while inserting `{0}`: `{1:?}`")]
    DbError(String, sqlx::Error),
    #[error("couldn't decode pcap file into a pcaket: {0}")]
    DecodePcapFile(String)
}
