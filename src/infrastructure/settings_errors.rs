//! Ошибки хранилища настроек

use std::io;

/// Ошибка хранилища настроек
#[derive(Debug)]
pub enum SettingsStorageError {
    Io(std::io::Error),
    Serialization(toml::ser::Error),
    Deserialization(toml::de::Error),
}

impl std::fmt::Display for SettingsStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsStorageError::Io(e) => write!(f, "I/O error: {}", e),
            SettingsStorageError::Serialization(e) => write!(f, "Serialization error: {}", e),
            SettingsStorageError::Deserialization(e) => write!(f, "Deserialization error: {}", e),
        }
    }
}

impl From<io::Error> for SettingsStorageError {
    fn from(e: io::Error) -> Self {
        SettingsStorageError::Io(e)
    }
}

impl From<toml::ser::Error> for SettingsStorageError {
    fn from(e: toml::ser::Error) -> Self {
        SettingsStorageError::Serialization(e)
    }
}

impl From<toml::de::Error> for SettingsStorageError {
    fn from(e: toml::de::Error) -> Self {
        SettingsStorageError::Deserialization(e)
    }
}
