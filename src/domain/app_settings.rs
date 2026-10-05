use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Настройки, которые приложение загружает при запуске
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AppSettings {
    pub(crate) logs_root: PathBuf,
}

pub(crate) trait RepoOfAppSettings<E> {
    fn save(&self, settings: &AppSettings) -> Result<(), E>;

    /// Загрузить настройки из файла
    ///
    /// Возвращает `None`, если файл не найден - это нормальная ситуация,
    /// когда пользователь ещё не настроил приложение.
    fn load(&self) -> Result<Option<AppSettings>, E>;
    fn delete(&self) -> Result<(), E>;
}
