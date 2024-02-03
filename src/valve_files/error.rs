use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValveFileError {
    #[error("error in keyvalues-serde: {0}")]
    KeyValuesSerde(String),
    #[error("error in keyvalues-parser: {0}")]
    KeyValuesParser(String),
    #[error("unknown error")]
    Unknown,
}
