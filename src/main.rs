mod application;
mod domain;
mod infrastructure;
mod presentation;

use crate::application::WeightExperiment;
use crate::domain::experiment::ExperimentsRunner;
use crate::infrastructure::{DefaultExperimentsRunner, FileRepoOfLogs};
use crate::presentation::{BarsPlotterByDays, Cli, SettingsInitializer};
use anyhow::Context;
use infrastructure::FileRepoOfAppSettings;
use std::io::stdin;

fn main() -> Result<(), anyhow::Error> {
    let repo_of_app_settings = FileRepoOfAppSettings::new();

    let settings_initializer = SettingsInitializer {
        repo_of_app_settings: Box::new(repo_of_app_settings),
        stdin: stdin(),
    };

    let settings = settings_initializer
        .load_or_ask()
        .context("failed to load settings")?;

    let mut experiments_runner = DefaultExperimentsRunner::new();
    experiments_runner.register_experiments(vec![Box::new(WeightExperiment::new(
        //todo разобраться кто когда куда определяет имя эксперимента
        Box::new(FileRepoOfLogs::new(settings.logs_root.as_ref(), "weight")),
        Box::new(BarsPlotterByDays::default()),
    ))]);
    let cli = Cli {
        stdin: stdin(),
        experiments_runner: Box::new(experiments_runner),
    };
    cli.run_mainloop()
}
