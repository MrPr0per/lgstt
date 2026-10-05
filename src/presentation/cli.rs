use crate::domain::RepoOfAppSettings;
use crate::domain::app_settings::AppSettings;
use colored::Colorize;
use std::fmt::{Debug, Display};
use std::io;
use std::io::{BufRead, Stdin};
use std::path::PathBuf;

pub(crate) struct Cli<RepoOfAppSettingsError> {
    pub(crate) repo_of_app_settings: Box<dyn RepoOfAppSettings<RepoOfAppSettingsError>>,
    pub(crate) stdin: Stdin,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ErrorOfCli<RepoOfAppSettingsError> {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Repo(RepoOfAppSettingsError),
}

impl<RepoOfAppSettingsError> Cli<RepoOfAppSettingsError>
where
    RepoOfAppSettingsError: Debug + Display,
{
    pub(crate) fn run_mainloop(&self) -> Result<(), ErrorOfCli<RepoOfAppSettingsError>> {
        self.load_or_ask_settings()?;

        println!("Enter events:");
        for line in self.stdin.lock().lines() {
            let line = line?;

            todo!()
        }
        Ok(())
    }

    fn load_or_ask_settings(&self) -> Result<AppSettings, ErrorOfCli<RepoOfAppSettingsError>> {
        let settings_or_not = self.repo_of_app_settings.load().map_err(ErrorOfCli::Repo)?;
        let settings = match settings_or_not {
            Some(s) => s,
            None => {
                let settings = self.ask_for_settings()?;
                self.repo_of_app_settings
                    .save(&settings)
                    .map_err(ErrorOfCli::Repo)?;
                settings
            }
        };
        Ok(settings)
    }

    pub(crate) fn ask_for_settings(&self) -> io::Result<AppSettings> {
        println!("Enter the path to the project:");
        let mut path = String::new();
        self.stdin.read_line(&mut path)?;
        Ok(AppSettings {
            logs_root: PathBuf::from(&path),
        })
    }
}
