//! Приложение для трекинга времени
//!
//! Слои:
//! - Domain: TimeSegment, ActivityType, ActivityInstance, AppSettings
//! - Infrastructure: FileRawEntryRepository, SettingsStorage
//! - Presentation: CLI интерфейс

mod domain;
mod infrastructure;
mod presentation;

use infrastructure::{FileRawEntryRepository, SettingsStorage};

fn main() {
    // Загрузка или создание настроек
    let settings_storage = SettingsStorage::new();
    let settings = match settings_storage.load() {
        Ok(Some(s)) => s,
        Ok(None) => {
            let settings = presentation::ask_for_settings();
            if let Err(e) = settings_storage.save(&settings) {
                panic!("Failed to save settings: {}", e);
            }
            settings
        }
        Err(e) => {
            panic!("Failed to load settings: {}", e);
        }
    };

    // Создание репозитория
    let repository = FileRawEntryRepository::new(settings.events_file_path);

    // Запуск основного цикла
    presentation::run_mainloop(&repository);
}