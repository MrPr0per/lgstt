use crate::domain::RepoOfAppSettings;
use crate::domain::app_settings::AppSettings;
use crate::domain::experiment::ExperimentsRunner;
use anyhow::Context;
use std::io::{BufRead, Stdin};
use std::path::PathBuf;

pub(crate) struct Cli {
    pub(crate) repo_of_app_settings: Box<dyn RepoOfAppSettings<anyhow::Error>>,
    pub(crate) stdin: Stdin,
    pub(crate) experiments_runner: Box<dyn ExperimentsRunner>,
}

impl Cli {
    pub(crate) fn run_mainloop(&self) -> anyhow::Result<()> {
        self.load_or_ask_settings()?;

        println!("Enter events:");
        for line in self.stdin.lock().lines() {
            let line = line.context("failed to read line from stdin")?;
            let (first_word, remainder) = line
                .split_once(' ')
                .context("failed to split line in first word and reminder")?;
            self.experiments_runner
                .run(first_word, remainder)
                .context("failed to run experiment")?;
        }
        Ok(())
    }

    fn load_or_ask_settings(&self) -> anyhow::Result<AppSettings> {
        let settings_if_exists = self
            .repo_of_app_settings
            .load()
            .context("failed to load app settings")?;
        let settings = match settings_if_exists {
            Some(s) => s,
            None => {
                let settings = self.ask_for_settings()?;
                self.repo_of_app_settings
                    .save(&settings)
                    .context("failed to save app settings")?;
                settings
            }
        };
        Ok(settings)
    }

    pub(crate) fn ask_for_settings(&self) -> anyhow::Result<AppSettings> {
        println!("Enter the path to the project:");
        let mut path = String::new();
        self.stdin
            .read_line(&mut path)
            .context("failed to read project path")?;
        Ok(AppSettings {
            logs_root: PathBuf::from(&path),
        })
    }
}
