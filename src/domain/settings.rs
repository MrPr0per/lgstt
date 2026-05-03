//! Settings Bounded Context - настройки приложения

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Настройки приложения
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Путь к файлу с событиями
    pub events_file_path: PathBuf,
}