use super::Error;
use crate::domain::{AppSettings, RepoOfAppSettings};
use directories::ProjectDirs;
use std::path::PathBuf;

pub struct Repo {
    config_file_path: PathBuf,
}

impl Repo {
    pub fn new() -> Self {
        Self {
            config_file_path: Self::default_path(),
        }
    }

    fn default_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .expect("Cannot determine config directory");
        proj_dirs.config_dir().join("config.toml")
    }
}

impl RepoOfAppSettings<Error> for Repo {
    fn save(&self, settings: &AppSettings) -> Result<(), Error> {
        if let Some(parent) = self.config_file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(settings)?;
        std::fs::write(&self.config_file_path, content)?;
        Ok(())
    }
    fn load(&self) -> Result<Option<AppSettings>, Error> {
        if !self.config_file_path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&self.config_file_path)?;
        let settings = toml::from_str(&content)?;
        Ok(Some(settings))
    }

    fn delete(&self) -> Result<(), Error> {
        if self.config_file_path.exists() {
            std::fs::remove_file(&self.config_file_path)?;
        }
        Ok(())
    }
}

impl Default for Repo {
    fn default() -> Self {
        Self::new()
    }
}
