//! Временной отрезок - основная единица трекинга времени

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::activity_instance::ActivityInstance;

/// Начатый временной отрезок
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StartedSegment {
    pub(crate) started_at: DateTime<Local>,
    pub(crate) plan: ActivityInstance,
}

/// Завершённый временной отрезок
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CompletedSegment {
    pub(crate) started_at: DateTime<Local>,
    pub(crate) ended_at: DateTime<Local>,
    pub(crate) plan: Option<ActivityInstance>,
    pub(crate) facts: Vec<ActivityInstance>,
}

/// Временной отрезок - основная единица трекинга времени
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum TimeSegment {
    Started(StartedSegment),
    Completed(CompletedSegment),
}

impl TimeSegment {
    pub(crate) fn new_started(started_at: DateTime<Local>, plan: ActivityInstance) -> Self {
        Self::Started(StartedSegment { started_at, plan })
    }

    pub(crate) fn new_completed(
        started_at: DateTime<Local>,
        ended_at: DateTime<Local>,
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
