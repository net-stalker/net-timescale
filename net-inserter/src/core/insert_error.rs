use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InsertError {
    #[error("the data for type `{0}` is wrong, bad decode")]
    WrongInsertableData(String),
    #[error("Error while inserting `{0}`: `{1:?}`")]
    DbError(String, Box<dyn std::error::Error + Send + Sync>),
    #[error("couldn't decode pcap file into a packet: {0}")]
    DecodePcapFile(String),
    #[error("cound't write file to a directory: {0}")]
    WriteFile(String)
}
