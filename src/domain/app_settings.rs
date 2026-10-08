use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Настройки, которые приложение загружает при запуске
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AppSettings {
    pub(crate) logs_root: PathBuf,
}

pub(crate) trait RepoOfAppSettings {
    fn save(&self, settings: &AppSettings) -> anyhow::Result<()>;

    /// Загрузить настройки из файла
    ///
    /// Возвращает `None`, если файл не найден - это нормальная ситуация,
    /// когда пользователь ещё не настроил приложение.
    fn load(&self) -> anyhow::Result<Option<AppSettings>>;
    fn delete(&self) -> anyhow::Result<()>;
}
