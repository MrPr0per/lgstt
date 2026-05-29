//! Экземпляр деятельности - конкретное использование типа

use serde::{Deserialize, Serialize};

use super::activity_type::ActivityTypeId;

/// Экземпляр деятельности - конкретное использование типа
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityInstance {
    pub type_id: ActivityTypeId,
    pub description: Option<String>,
}

impl ActivityInstance {
    pub fn new(type_id: ActivityTypeId, description: Option<String>) -> Self {
        Self {
            type_id,
            description,
        }
    }
}