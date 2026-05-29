// привязанные к контекстам штуки в соответсвующих модулях

mod app_settings;
mod raw_entries;

pub(crate) use app_settings::FileRepo as FileRepoOfAppSettings;
pub(crate) use raw_entries::FileRepo as FileRepoOfRawEntries;
