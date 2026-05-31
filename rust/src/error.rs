use std::io;
use std::fmt;

#[derive(Debug)]
pub enum MmfgError {
    Io(io::Error),
    Shm(String),
    Protocol(String),
    Registry(String),
    Expansion(String),
    Internal(String),
}

impl fmt::Display for MmfgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MmfgError::Io(e) => write!(f, "IO error: {}", e),
            MmfgError::Shm(s) => write!(f, "SHM error: {}", s),
            MmfgError::Protocol(s) => write!(f, "Protocol error: {}", s),
            MmfgError::Registry(s) => write!(f, "Registry error: {}", s),
            MmfgError::Expansion(s) => write!(f, "Expansion error: {}", s),
            MmfgError::Internal(s) => write!(f, "Internal error: {}", s),
        }
    }
}

impl std::error::Error for MmfgError {}

impl From<io::Error> for MmfgError {
    fn from(err: io::Error) -> Self {
        MmfgError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, MmfgError>;
