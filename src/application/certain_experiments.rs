use crate::domain::experiment::{Experiment, RepoOfLogs};
use crate::presentation::plots::{ChartPlotter, DateTimePoint};
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct PayloadOfWeightExperiment {
    weight: f32,
}

pub(crate) struct WeightExperiment {
    repo: Box<dyn RepoOfLogs<PayloadOfWeightExperiment>>,
    chart_plotter: Box<dyn ChartPlotter>,
}

impl WeightExperiment {
    pub(crate) fn new(
        repo: Box<dyn RepoOfLogs<PayloadOfWeightExperiment>>,
        chart_plotter: Box<dyn ChartPlotter>,
    ) -> WeightExperiment {
        WeightExperiment {
            repo,
            chart_plotter,
        }
    }
}

impl Experiment for WeightExperiment {
    fn name(&self) -> &'static str {
        "weight"
    }

    fn run(&self, args: &str) -> anyhow::Result<()> {
        let weight = args
            .trim()
            .parse::<f32>()
            .with_context(|| format!("Failed to parse weight from {args:?}"))?;

        self.repo
            .save_to_the_end(PayloadOfWeightExperiment { weight })
            .context("Failed to save weight log")?;

        let logs = self.repo.load().context("Failed to load weight logs")?;

        self.chart_plotter
            .plot(
                logs.iter()
                    .map(|x| DateTimePoint {
                        datetime: x.datetime,
                        value: x.payload.weight,
                    })
                    .collect(),
            )
            .context("Failed to plot")?;

        Ok(())
    }
}
