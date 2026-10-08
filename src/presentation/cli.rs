use crate::domain::experiment::ExperimentsRunner;
use anyhow::Context;
use std::io::{BufRead, Stdin};

pub(crate) struct Cli {
    pub(crate) stdin: Stdin,
    pub(crate) experiments_runner: Box<dyn ExperimentsRunner>,
}

impl Cli {
    pub(crate) fn run_mainloop(&self) -> anyhow::Result<()> {
        println!("Enter events:");
        for line in self.stdin.lock().lines() {
            let line = line.context("failed to read line from stdin")?;
            let (first_word, remainder) = line
                .split_once(' ')
                .context("failed to split line in first word and remainder")?;
            self.experiments_runner
                .run(first_word, remainder)
                .context("failed to run experiment")?;
        }
        Ok(())
    }
}