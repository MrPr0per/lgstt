//! Сырая запись - оригинальный ввод пользователя

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Сырая запись - оригинальный ввод пользователя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEntry {
    pub timestamp: DateTime<Local>,
    pub content: String,
}

impl RawEntry {
    pub fn new(raw_input: String) -> Self {
        Self {
            timestamp: Local::now(),
            content: raw_input,
        }
    }
}