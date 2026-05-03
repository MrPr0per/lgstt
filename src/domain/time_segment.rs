//! Временной отрезок - основная единица трекинга времени

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::activity_instance::ActivityInstance;

/// Начатый временной отрезок
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartedSegment {
    pub started_at: DateTime<Utc>,
    pub plan: ActivityInstance,
}

/// Завершённый временной отрезок
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedSegment {
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub plan: Option<ActivityInstance>,
    pub facts: Vec<ActivityInstance>,
}

/// Временной отрезок - основная единица трекинга времени
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeSegment {
    Started(StartedSegment),
    Completed(CompletedSegment),
}

impl TimeSegment {
    pub fn new_started(started_at: DateTime<Utc>, plan: ActivityInstance) -> Self {
        Self::Started(StartedSegment { started_at, plan })
    }

    pub fn new_completed(
        started_at: DateTime<Utc>,
        ended_at: DateTime<Utc>,
        plan: Option<ActivityInstance>,
        facts: Vec<ActivityInstance>,
    ) -> Self {
        Self::Completed(CompletedSegment {
            started_at,
            ended_at,
            plan,
            facts,
        })
    }
}