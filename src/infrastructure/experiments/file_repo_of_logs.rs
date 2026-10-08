use crate::domain::experiment::{LogInfo, RepoOfLogs};
use anyhow::Context;
use serde::{Serialize, de::DeserializeOwned};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub(crate) struct FileRepoOfLogs {
    path_to_file: PathBuf,
}

impl FileRepoOfLogs {
    /// `logs_root` - папка из настроек, `name` - имя файла без расширения
    pub(crate) fn new(logs_root: &Path, name: &str) -> Self {
        Self {
            path_to_file: logs_root.join(format!("{name}.jsonl")),
        }
    }
}

impl<TPayload> RepoOfLogs<TPayload> for FileRepoOfLogs
where
    TPayload: Serialize + DeserializeOwned,
{
    fn load(&self) -> anyhow::Result<Vec<LogInfo<TPayload>>> {
        if !self.path_to_file.exists() {
            return Ok(Vec::new());
        }
        let file = std::fs::File::open(&self.path_to_file)
            .with_context(|| format!("cannot open {}", self.path_to_file.display()))?;

        let mut logs = Vec::new();
        for (index, line) in BufReader::new(file).lines().enumerate() {
            let line = line.context("failed to read line")?;
            if line.trim().is_empty() {
                continue;
            }
            let log = serde_json::from_str(&line).with_context(|| {
                format!(
                    "{}: bad json on line {}",
                    self.path_to_file.display(),
                    index + 1
                )
            })?;
            logs.push(log);
        }
        Ok(logs)
    }

    fn save_to_the_end(&self, payload: TPayload) -> anyhow::Result<()> {
        if let Some(parent) = self.path_to_file.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("failed to create dirs to {}", self.path_to_file.display())
            })?;
        }
        let log = LogInfo::new(payload);

        let mut line = serde_json::to_string(&log)?;
        line.push('\n');

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path_to_file)
            .with_context(|| format!("cannot open {}", self.path_to_file.display()))?;
        file.write_all(line.as_bytes())?;
        Ok(())
    }
}
