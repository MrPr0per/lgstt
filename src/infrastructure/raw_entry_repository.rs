//! Файловый репозиторий для сырых записей

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use super::errors::FileRepositoryError;
use super::raw_entry::RawEntry;

/// Файловый репозиторий для сырых записей
pub struct FileRawEntryRepository {
    pub file_path: PathBuf,
}

impl FileRawEntryRepository {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    fn ensure_file_exists(&self) -> Result<(), FileRepositoryError> {
        if let Some(parent) = self.file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        Ok(())
    }
}

impl FileRawEntryRepository {
    pub fn save(&self, entry: &RawEntry) -> Result<(), FileRepositoryError> {
        self.ensure_file_exists()?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        let json = serde_json::to_string(entry)?;
        writeln!(file, "{json}")?;
        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<RawEntry>, FileRepositoryError> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<RawEntry>(&line) {
                Ok(entry) => entries.push(entry),
                Err(e) => eprintln!("Failed to parse line: {e}"),
            }
        }
        Ok(entries)
    }
}