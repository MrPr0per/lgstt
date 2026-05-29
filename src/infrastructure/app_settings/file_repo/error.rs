//! Ошибки хранилища настроек

use std::io;

/// Ошибка хранилища настроек
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Serialization(toml::ser::Error),
    Deserialization(toml::de::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Serialization(e) => write!(f, "Serialization error: {}", e),
            Error::Deserialization(e) => write!(f, "Deserialization error: {}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error::Serialization(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::Deserialization(e)
    }
}
