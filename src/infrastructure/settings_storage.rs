//! Хранилище настроек приложения

use crate::domain::settings::AppSettings;
use directories::ProjectDirs;
use std::path::PathBuf;

use super::settings_errors::SettingsStorageError;

/// Хранилище настроек приложения
pub struct SettingsStorage {
    /// Путь к файлу конфигурации
    config_file_path: PathBuf,
}

impl SettingsStorage {
    pub fn new() -> Self {
        Self {
            config_file_path: Self::default_path(),
        }
    }

    fn default_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .expect("Cannot determine config directory");
        proj_dirs.config_dir().join("config.toml")
    }

    /// Загрузить настройки из файла
    ///
    /// Возвращает `None`, если файл не найден - это нормальная ситуация,
    /// когда пользователь ещё не настроил приложение.
    pub fn load(&self) -> Result<Option<AppSettings>, SettingsStorageError> {
        if !self.config_file_path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&self.config_file_path)?;
        let settings = toml::from_str(&content)?;
        Ok(Some(settings))
    }

    /// Сохранить настройки в файл
    pub fn save(&self, settings: &AppSettings) -> Result<(), SettingsStorageError> {
        if let Some(parent) = self.config_file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(settings)?;
        std::fs::write(&self.config_file_path, content)?;
        Ok(())
    }
}

impl Default for SettingsStorage {
    fn default() -> Self {
        Self::new()
    }
}