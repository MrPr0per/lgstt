//! Тип деятельности - классификатор для группировки занятий

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::tag::Tag;

/// Идентификатор типа деятельности
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActivityTypeId(Uuid);

impl ActivityTypeId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for ActivityTypeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ActivityTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Тип деятельности - классификатор для группировки занятий
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityType {
    pub id: ActivityTypeId,
    pub canonical_name: String,
    pub aliases: Vec<String>,
    pub tags: Vec<Tag>,
}

impl ActivityType {
    pub fn new(canonical_name: String) -> Self {
        Self {
            id: ActivityTypeId::new(),
            canonical_name,
            aliases: Vec::new(),
            tags: Vec::new(),
        }
    }

    /// Проверить, соответствует ли строка этому типу (по canonical_name или alias)
    pub fn matches(&self, input: &str) -> bool {
        let input_lower = input.to_lowercase();
        self.canonical_name.to_lowercase() == input_lower
            || self.aliases.iter().any(|a| a.to_lowercase() == input_lower)
    }
}