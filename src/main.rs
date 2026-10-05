mod domain;
mod infrastructure;
mod presentation;

use crate::infrastructure::ErrorOfFileRepoOfAppSettings;
use crate::presentation::Cli;
use crate::presentation::ErrorOfCli;
use infrastructure::FileRepoOfAppSettings;
use std::io::stdin;

fn main() -> Result<(), ErrorOfCli<ErrorOfFileRepoOfAppSettings>> {
    let repo_of_app_settings = FileRepoOfAppSettings::new();
    let cli = Cli {
        repo_of_app_settings: Box::new(repo_of_app_settings),
        stdin: stdin(),
    };
    cli.run_mainloop()
}
