use crate::domain::{AppSettings, RepoOfAppSettings};
use directories::ProjectDirs;
use std::path::PathBuf;

pub(crate) struct FileRepoOfAppSettings {
    path_to_settings: PathBuf,
}
#[derive(Debug, thiserror::Error)]
pub(crate) enum ErrorOfFileRepoOfAppSettings {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl FileRepoOfAppSettings {
    pub(crate) fn new() -> Self {
        Self {
            path_to_settings: Self::default_path(),
        }
    }

    fn default_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .expect("Cannot determine config directory");
        proj_dirs.config_dir().join("app_settings.toml")
    }
}

impl RepoOfAppSettings<ErrorOfFileRepoOfAppSettings> for FileRepoOfAppSettings {
    fn save(&self, settings: &AppSettings) -> Result<(), ErrorOfFileRepoOfAppSettings> {
        if let Some(parent) = self.path_to_settings.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(settings)?;
        std::fs::write(&self.path_to_settings, content)?;
        Ok(())
    }
    fn load(&self) -> Result<Option<AppSettings>, ErrorOfFileRepoOfAppSettings> {
        if !self.path_to_settings.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&self.path_to_settings)?;
        let settings = serde_json::from_str(&content)?;
        Ok(Some(settings))
    }

    fn delete(&self) -> Result<(), ErrorOfFileRepoOfAppSettings> {
        if self.path_to_settings.exists() {
            std::fs::remove_file(&self.path_to_settings)?;
        }
        Ok(())
    }
}
