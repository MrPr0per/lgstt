//! Экземпляр деятельности - конкретное использование типа

use serde::{Deserialize, Serialize};

use super::activity_type::ActivityTypeId;

/// Экземпляр деятельности - конкретное использование типа
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ActivityInstance {
    pub(crate) type_id: ActivityTypeId,
    pub(crate) description: Option<String>,
}

impl ActivityInstance {
    pub(crate) fn new(type_id: ActivityTypeId, description: Option<String>) -> Self {
        Self {
            type_id,
            description,
        }
    }
}
