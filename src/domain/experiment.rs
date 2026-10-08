use chrono::{DateTime, Local};

pub(crate) trait ExperimentsRunner {
    fn register_experiments(&self, experiments: Vec<Box<dyn Experiment>>);
    fn run(&self, prefix_of_name_of_experiment: &str, args: &str) -> anyhow::Result<()>;
}

pub(crate) trait Experiment {
    fn name(&self) -> &'static str;
    fn run(&self, args: &str) -> anyhow::Result<()>;
}

pub(crate) struct LogInfo<TPayload> {
    pub(crate) datetime: DateTime<Local>,
    pub(crate) payload: TPayload,
}

pub(crate) trait RepoOfLogs<TPayload> {
    fn load(&self) -> anyhow::Result<Vec<LogInfo<TPayload>>>;
    fn save_to_end(&self, payload: TPayload) -> anyhow::Result<()>;
}

impl<TPayload> LogInfo<TPayload> {
    pub(crate) fn new(payload: TPayload) -> Self {
        Self {
            datetime: Local::now(),
            payload,
        }
    }
}
