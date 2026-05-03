//! CLI интерфейс для ввода событий

use colored::Colorize;
use crate::domain::settings::AppSettings;
use crate::infrastructure::{FileRawEntryRepository, RawEntry};
use std::io;
use std::path::PathBuf;

/// Запросить настройки у пользователя (если их нет)
pub fn ask_for_settings() -> AppSettings {
    println!("Enter the path to the project (empty string = current directory)");
    let mut path_string = String::new();
    io::stdin()
        .read_line(&mut path_string)
        .expect("Failed to read string");
    path_string = path_string.trim().to_string();
    if path_string.is_empty() {
        path_string = ".".to_string();
    }
    AppSettings {
        events_file_path: PathBuf::from(&path_string).join("events.jsons"),
    }
}

/// Запуск основного цикла CLI
pub fn run_mainloop(repository: &FileRawEntryRepository) {
    let stdin = io::stdin();

    println!("Enter events:");
    for line in stdin.lines() {
        let line = line.expect("Failed to read string");

        // Создаём сырую запись
        let entry = RawEntry::new(line.clone());

        // Сохраняем
        if let Err(e) = repository.save(&entry) {
            eprintln!("Failed to save entry: {}", e);
            continue;
        }

        // Выводим подтверждение
        println!(
            "{} {}",
            entry
                .timestamp
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .as_str()
                .green(),
            entry.content
        );
    }
}
