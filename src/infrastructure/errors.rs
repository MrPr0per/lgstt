//! Ошибки инфраструктуры

use serde_json;
use std::io;

/// Ошибка файлового репозитория
#[derive(Debug)]
pub enum FileRepositoryError {
    Io(io::Error),
    Serialization(serde_json::Error),
}

impl std::fmt::Display for FileRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileRepositoryError::Io(e) => write!(f, "I/O error: {}", e),
            FileRepositoryError::Serialization(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl From<io::Error> for FileRepositoryError {
    fn from(e: io::Error) -> Self {
        FileRepositoryError::Io(e)
    }
}

impl From<serde_json::Error> for FileRepositoryError {
    fn from(e: serde_json::Error) -> Self {
        FileRepositoryError::Serialization(e)
    }
}