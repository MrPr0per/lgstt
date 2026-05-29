use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub events_file_path: PathBuf,
}

pub trait Repo<E> {
    fn save(&self, settings: &Settings) -> Result<(), E>;

    /// Загрузить настройки из файла
    ///
    /// Возвращает `None`, если файл не найден - это нормальная ситуация,
    /// когда пользователь ещё не настроил приложение.
    fn load(&self) -> Result<Option<Settings>, E>;
    fn delete(&self) -> Result<(), E>;
}
