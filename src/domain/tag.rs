//! Теги с иерархической структурой

use serde::{Deserialize, Serialize};

/// Тег с иерархической структурой (например, полезное::учеба::го)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub path: Vec<String>,
}

impl Tag {
    /// Создать тег из пути
    pub fn from_path(path: Vec<String>) -> Self {
        Self { path }
    }

    /// Создать тег из строки с разделителем "::"
    pub fn from_str(s: &str) -> Self {
        Self {
            path: s.split("::").map(|s| s.to_string()).collect(),
        }
    }

    /// Получить строковое представление
    pub fn to_string(&self) -> String {
        self.path.join("::")
    }
}
