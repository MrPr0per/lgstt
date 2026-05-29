// каждый модуль в domain - это bounded context
pub mod app_settings;
pub mod raw_entry;
pub mod time_segments;

pub(crate) use app_settings::Repo as RepoOfAppSettings;
pub(crate) use app_settings::Settings as AppSettings;
pub(crate) use raw_entry::RawEntry;
