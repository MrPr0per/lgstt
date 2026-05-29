//! Ошибки инфраструктуры

use serde_json;
use std::io;

/// Ошибка файлового репозитория
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Serialization(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Serialization(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serialization(e)
    }
}
