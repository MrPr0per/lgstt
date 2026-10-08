use crate::domain::experiment::{Experiment, ExperimentsRunner};
use anyhow::{anyhow, bail};

pub(crate) struct DefaultExperimentsRunner {
    experiments: Vec<Box<dyn Experiment>>,
}

impl DefaultExperimentsRunner {
    pub(crate) fn new() -> Self {
        Self {
            experiments: Vec::new(),
        }
    }
}

impl Default for DefaultExperimentsRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl ExperimentsRunner for DefaultExperimentsRunner {
    fn register_experiments(&mut self, experiments: Vec<Box<dyn Experiment>>) {
        for checking_experiment in experiments {
            assert!(
                self.experiments
                    .iter()
                    .all(|e| e.name() != checking_experiment.name()),
                "experiment '{}' is already registered",
                checking_experiment.name()
            );
            self.experiments.push(checking_experiment);
        }
    }

    fn run(&self, prefix_of_name_of_experiment: &str, args: &str) -> anyhow::Result<()> {
        let matched: Vec<&Box<dyn Experiment>> = self
            .experiments
            .iter()
            .filter(|e| e.name().starts_with(prefix_of_name_of_experiment))
            .collect();

        match matched.as_slice() {
            [] => Err(anyhow!(
                "no experiment matches prefix '{}'. Available: [{}]",
                prefix_of_name_of_experiment,
                {
                    let mut names: Vec<&str> = self.experiments.iter().map(|e| e.name()).collect();
                    names.sort();
                    names.join(", ")
                }
            )),
            [single] => single.run(args),
            many => {
                // Точное совпадение имени имеет приоритет над совпадением по префиксу
                if let Some(exact) = many
                    .iter()
                    .find(|e| e.name() == prefix_of_name_of_experiment)
                {
                    return exact.run(args);
                }

                let names: Vec<&str> = many.iter().map(|e| e.name()).collect();
                bail!(
                    "prefix '{}' is ambiguous, matches: [{}]",
                    prefix_of_name_of_experiment,
                    names.join(", ")
                )
            }
        }
    }
}
