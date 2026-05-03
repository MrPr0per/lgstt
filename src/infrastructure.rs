//! Infrastructure Layer - работа с внешними системами (файлы, БД, API)
//!
//! Содержит:
//! - RawEntry - сырые данные
//! - FileRawEntryRepository - файловый репозиторий для сырых записей
//! - SettingsStorage - хранилище настроек
//! - FileRepositoryError, SettingsStorageError - ошибки

mod errors;
mod raw_entry;
mod raw_entry_repository;
mod settings_errors;
mod settings_storage;

pub use errors::FileRepositoryError;
pub use raw_entry::RawEntry;
pub use raw_entry_repository::FileRawEntryRepository;
pub use settings_errors::SettingsStorageError;
pub use settings_storage::SettingsStorage;
