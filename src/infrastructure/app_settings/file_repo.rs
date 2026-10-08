use crate::domain::{AppSettings, RepoOfAppSettings};
use anyhow::Context;
use directories::ProjectDirs;
use std::path::PathBuf;

pub(crate) struct FileRepoOfAppSettings {
    path_to_settings: PathBuf,
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
        proj_dirs.config_dir().join("app_settings.json")
    }
}

impl RepoOfAppSettings for FileRepoOfAppSettings {
    fn save(&self, settings: &AppSettings) -> anyhow::Result<()> {
        if let Some(parent) = self.path_to_settings.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create dirs to {}",
                    self.path_to_settings.display()
                )
            })?;
        }
        let content = serde_json::to_string_pretty(settings).context("failed to serialise")?;
        std::fs::write(&self.path_to_settings, content).context("failed to write in file")?;
        Ok(())
    }
    fn load(&self) -> anyhow::Result<Option<AppSettings>> {
        if !self.path_to_settings.exists() {
            return Ok(None);
        }
        let content =
            std::fs::read_to_string(&self.path_to_settings).context("failed to read file")?;
        let settings = serde_json::from_str(&content)
            .with_context(|| format!("failed to parse: {}", content))?;
        Ok(Some(settings))
    }

    fn delete(&self) -> anyhow::Result<()> {
        if self.path_to_settings.exists() {
            std::fs::remove_file(&self.path_to_settings).context("failed to remove file")?;
        }
        Ok(())
    }
}
