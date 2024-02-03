use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SteamAppLauncherError {
    #[error("unknown error")]
    Unknown,
    #[error("steam library not found")]
    LibraryNotFound,
    #[error("file not readable")]
    Read(#[from] io::Error),
    #[error("failed to parse file: {0}")]
    Parse(String),
}
