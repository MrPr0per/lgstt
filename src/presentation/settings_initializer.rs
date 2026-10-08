use crate::domain::RepoOfAppSettings;
use crate::domain::app_settings::AppSettings;
use anyhow::Context;
use std::io::Stdin;
use std::path::PathBuf;

pub(crate) struct SettingsInitializer {
    pub(crate) repo_of_app_settings: Box<dyn RepoOfAppSettings>,
    pub(crate) stdin: Stdin,
}

impl SettingsInitializer {
    pub(crate) fn load_or_ask(&self) -> anyhow::Result<AppSettings> {
        let settings_if_exists = self
            .repo_of_app_settings
            .load()
            .context("failed to load app settings")?;
        let settings = match settings_if_exists {
            Some(s) => s,
            None => {
                let settings = self.ask()?;
                self.repo_of_app_settings
                    .save(&settings)
                    .context("failed to save app settings")?;
                settings
            }
        };
        Ok(settings)
    }

    fn ask(&self) -> anyhow::Result<AppSettings> {
        println!("Enter the path to the project:");
        let mut path = String::new();
        self.stdin
            .read_line(&mut path)
            .context("failed to read project path")?;
        Ok(AppSettings {
            logs_root: PathBuf::from(path.trim()),
        })
    }
}
