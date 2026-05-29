mod domain;
mod infrastructure;
mod presentation;

use crate::domain::RepoOfAppSettings;
use infrastructure::{FileRepoOfAppSettings, FileRepoOfRawEntries};

fn main() {
    // Загрузка или создание настроек
    let settings_repo = FileRepoOfAppSettings::new();
    let settings = match settings_repo.load() {
        Ok(Some(s)) => s,
        Ok(None) => {
            let settings = presentation::ask_for_settings();
            if let Err(e) = settings_repo.save(&settings) {
                panic!("Failed to save settings: {}", e);
            }
            settings
        }
        Err(e) => {
            panic!("Failed to load settings: {}", e);
        }
    };

    // Создание репозитория
    let repository = FileRepoOfRawEntries::new(settings.events_file_path);

    // Запуск основного цикла
    presentation::run_mainloop(&repository);
}
